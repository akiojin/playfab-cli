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

// Re-export the canonical ToolSpec and related types from tool_executor
pub use super::tool_executor::{AuthMode, RetryMode, ToolCategory, ToolSpec};

/// Returns all tool specs from every catalog module.
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
