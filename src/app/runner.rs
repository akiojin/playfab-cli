use anyhow::{anyhow, Context, Result};
use clap::Parser;
use serde_json::{json, Value};
use std::fs;
use tracing_subscriber::EnvFilter;

use crate::cli::{
    Cli, CliCommand, Command, ConfigCommand, OutputFormat, RawArgs, SystemCommand, ToolCommand,
};
use crate::core::config::PlayFabConfig;
use crate::http::auth::AuthManager;
use crate::http::client::PlayFabClient;
use crate::tooling::catalog;
use crate::tooling::tool_executor;

pub async fn run() -> Result<()> {
    let cli = Cli::parse();
    run_with_cli(cli).await
}

pub async fn run_with_cli(cli: Cli) -> Result<()> {
    init_tracing(cli.verbose)?;

    match &cli.command {
        Command::Tool { command } => match command {
            ToolCommand::List => {
                let tools = catalog::all_tools();
                if matches!(cli.output, OutputFormat::Json) {
                    let names: Vec<&str> = tools.iter().map(|t| t.name.as_str()).collect();
                    print_value(&serde_json::to_value(names)?, cli.output)?;
                } else {
                    for tool in &tools {
                        println!("{:<40} {}", tool.name, tool.description);
                    }
                }
            }
            ToolCommand::Schema { tool_name } => {
                let tools = catalog::all_tools();
                if let Some(name) = tool_name {
                    let tool = catalog::find_tool(&tools, name)
                        .ok_or_else(|| anyhow!("Unknown tool: {}", name))?;
                    print_value(&serde_json::to_value(tool)?, cli.output)?;
                } else {
                    let schemas: Vec<Value> = tools
                        .iter()
                        .map(|t| serde_json::to_value(t).unwrap_or_default())
                        .collect();
                    print_value(&Value::Array(schemas), cli.output)?;
                }
            }
            ToolCommand::Call(args) => {
                let value = execute_tool_command(&cli, args)?;
                print_value(&value, cli.output)?;
            }
            ToolCommand::External(args) => {
                if args.is_empty() {
                    return Err(anyhow!("No tool name provided"));
                }
                let tool_name = &args[0];
                let json_str = args.get(1).cloned();
                let raw_args = RawArgs {
                    tool_name: tool_name.clone(),
                    json: json_str,
                    params_file: None,
                };
                let value = execute_tool_command(&cli, &raw_args)?;
                print_value(&value, cli.output)?;
            }
        },
        Command::System { command } => match command {
            SystemCommand::Ping => {
                let config = PlayFabConfig::from_env()?;
                let result = json!({
                    "status": "ok",
                    "title_id": config.title_id,
                    "api_endpoint": config.api_endpoint,
                });
                print_value(&result, cli.output)?;
            }
        },
        Command::Config { command } => match command {
            ConfigCommand::Show => match PlayFabConfig::from_env() {
                Ok(config) => {
                    let result = json!({
                        "title_id": config.title_id,
                        "api_endpoint": config.api_endpoint,
                        "secret_key_set": !config.dev_secret_key.is_empty(),
                    });
                    print_value(&result, cli.output)?;
                }
                Err(e) => {
                    let result = json!({
                        "error": format!("{}", e),
                        "hint": "Set PLAYFAB_TITLE_ID and PLAYFAB_DEV_SECRET_KEY environment variables"
                    });
                    print_value(&result, cli.output)?;
                }
            },
            ConfigCommand::Set { key, value } => {
                eprintln!("Config set is not yet implemented. Use environment variables.");
                eprintln!("  {}={}", key, value);
            }
        },
        Command::Batch { json, stdin } => {
            let input = if *stdin {
                let mut buf = String::new();
                std::io::Read::read_to_string(&mut std::io::stdin(), &mut buf)?;
                buf
            } else if let Some(json_str) = json {
                json_str.clone()
            } else {
                return Err(anyhow!("Batch requires --json or --stdin"));
            };

            let batch: Vec<Value> =
                serde_json::from_str(&input).context("Failed to parse batch JSON")?;

            let config = PlayFabConfig::from_env()?;
            let client = PlayFabClient::new();
            let auth = AuthManager::new(config.clone());
            let tools = catalog::all_tools();

            let mut results = Vec::new();
            for item in &batch {
                let tool_name = item.get("tool").and_then(|v| v.as_str()).unwrap_or("");
                let params = item.get("params").cloned().unwrap_or(json!({}));

                match catalog::find_tool(&tools, tool_name) {
                    Some(tool) => {
                        match tool_executor::execute_tool(tool, params, &config, &client, &auth) {
                            Ok(result) => results.push(json!({
                                "tool": tool_name,
                                "success": true,
                                "result": result
                            })),
                            Err(e) => results.push(json!({
                                "tool": tool_name,
                                "success": false,
                                "error": format!("{}", e)
                            })),
                        }
                    }
                    None => results.push(json!({
                        "tool": tool_name,
                        "success": false,
                        "error": "Unknown tool"
                    })),
                }
            }
            print_value(&Value::Array(results), cli.output)?;
        }
        Command::Cli { command } => match command {
            CliCommand::Install { force: _ } => {
                eprintln!("CLI install not yet implemented");
            }
            CliCommand::Doctor => {
                let config_status = match PlayFabConfig::from_env() {
                    Ok(c) => json!({"status": "ok", "title_id": c.title_id}),
                    Err(e) => json!({"status": "error", "error": format!("{}", e)}),
                };
                let result = json!({
                    "version": env!("CARGO_PKG_VERSION"),
                    "config": config_status,
                });
                print_value(&result, cli.output)?;
            }
        },
    }

    Ok(())
}

fn execute_tool_command(_cli: &Cli, args: &RawArgs) -> Result<Value> {
    let params: Value = if let Some(json_str) = &args.json {
        serde_json::from_str(json_str).context("Failed to parse --json parameter")?
    } else if let Some(path) = &args.params_file {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read params file: {}", path.display()))?;
        serde_json::from_str(&content).context("Failed to parse params file as JSON")?
    } else {
        json!({})
    };

    let tools = catalog::all_tools();

    let tool = catalog::find_tool(&tools, &args.tool_name)
        .ok_or_else(|| anyhow!("Unknown tool: {}", args.tool_name))?;

    let config = PlayFabConfig::from_env()?;
    let client = PlayFabClient::new();
    let auth = AuthManager::new(config.clone());

    tool_executor::execute_tool(tool, params, &config, &client, &auth)
}

fn print_value(value: &Value, format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(value)?);
        }
        OutputFormat::Text => {
            println!("{}", serde_json::to_string_pretty(value)?);
        }
    }
    Ok(())
}

fn init_tracing(verbose: u8) -> Result<()> {
    let filter = match verbose {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(filter)),
        )
        .with_writer(std::io::stderr)
        .try_init()
        .ok();

    Ok(())
}
