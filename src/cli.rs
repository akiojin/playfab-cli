use std::path::PathBuf;

use clap::{ArgAction, Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum OutputFormat {
    #[default]
    Text,
    Json,
}

#[derive(Debug, Parser)]
#[command(
    name = "playfab-cli",
    version,
    about = "Rust CLI for PlayFab service automation",
    arg_required_else_help = true
)]
pub struct Cli {
    #[arg(long, global = true, value_enum, default_value_t = OutputFormat::Text)]
    pub output: OutputFormat,

    #[arg(short, long, global = true, action = ArgAction::Count)]
    pub verbose: u8,

    #[arg(long, global = true)]
    pub dry_run: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Execute a tool by name
    Tool {
        #[command(subcommand)]
        command: ToolCommand,
    },
    /// System commands
    System {
        #[command(subcommand)]
        command: SystemCommand,
    },
    /// Configuration management
    Config {
        #[command(subcommand)]
        command: ConfigCommand,
    },
    /// Execute batch operations
    Batch {
        #[arg(long, value_name = "JSON")]
        json: Option<String>,
        #[arg(long)]
        stdin: bool,
    },
    /// CLI management
    Cli {
        #[command(subcommand)]
        command: CliCommand,
    },
}

#[derive(Debug, Subcommand)]
pub enum ToolCommand {
    /// List all available tools
    List,
    /// Show JSON schema for a tool
    Schema {
        tool_name: Option<String>,
    },
    /// Call a tool with parameters
    Call(RawArgs),
    #[command(external_subcommand)]
    External(Vec<String>),
}

#[derive(Debug, Args)]
pub struct RawArgs {
    pub tool_name: String,

    #[arg(long, value_name = "JSON")]
    pub json: Option<String>,

    #[arg(long, value_name = "FILE")]
    pub params_file: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
pub enum SystemCommand {
    /// Check connectivity
    Ping,
}

#[derive(Debug, Subcommand)]
pub enum ConfigCommand {
    /// Show current configuration
    Show,
    /// Set a configuration value
    Set {
        key: String,
        value: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum CliCommand {
    /// Install or update the CLI binary
    Install {
        #[arg(long, default_value_t = false)]
        force: bool,
    },
    /// Check CLI health
    Doctor,
}
