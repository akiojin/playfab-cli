pub mod admin;
pub mod authentication;
pub mod cloudscript;
pub mod data;
pub mod economy_catalog;
pub mod economy_inventory;
pub mod events;
pub mod experimentation;
pub mod groups;
pub mod multiplayer;
pub mod profiles;
pub mod progression;
pub mod server;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Minimal ToolSpec definition used by the catalog and runner.
/// Unit 4 will provide the full version in `tool_executor.rs`; once that
/// lands the catalog modules will be updated to re-export from there.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolSpec {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub parameters: Value,
}

/// Returns all tool specs from every catalog module.
/// Populated by Unit 13.
pub fn all_tools() -> Vec<ToolSpec> {
    let mut tools = Vec::new();
    tools.extend(admin::tools());
    tools.extend(authentication::tools());
    tools.extend(cloudscript::tools());
    tools.extend(data::tools());
    tools.extend(economy_catalog::tools());
    tools.extend(economy_inventory::tools());
    tools.extend(events::tools());
    tools.extend(experimentation::tools());
    tools.extend(groups::tools());
    tools.extend(multiplayer::tools());
    tools.extend(profiles::tools());
    tools.extend(progression::tools());
    tools.extend(server::tools());
    tools
}

/// Find a tool by name in the given tool list.
pub fn find_tool<'a>(tools: &'a [ToolSpec], name: &str) -> Option<&'a ToolSpec> {
    tools.iter().find(|t| t.name == name)
}

/// Execute a tool with the given parameters.
/// This is a temporary stub that will be replaced by the full implementation in Unit 4.
pub fn execute_tool(
    tool: &ToolSpec,
    params: Value,
    _config: &crate::core::config::PlayFabConfig,
    _client: &crate::http::client::PlayFabClient,
    _auth: &crate::http::auth::AuthManager,
) -> anyhow::Result<Value> {
    Ok(serde_json::json!({
        "tool": tool.name,
        "status": "not_implemented",
        "params": params,
        "message": "Tool execution engine not yet implemented (Unit 4)"
    }))
}
