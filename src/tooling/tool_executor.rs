use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Which PlayFab API group a tool belongs to
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolCategory {
    EconomyCatalog,
    EconomyInventory,
    Admin,
    Server,
    Authentication,
    Profiles,
    Data,
    CloudScript,
    Events,
    Groups,
    Progression,
    Multiplayer,
    Experimentation,
}

/// Authentication method for API calls
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthMode {
    /// Uses X-SecretKey header (admin/server APIs)
    SecretKey,
    /// Uses X-EntityToken header (entity-based APIs)
    EntityToken,
    /// No authentication required
    None,
}

/// Retry behavior tier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetryMode {
    Strict,
    Standard,
    Bulk,
}

impl RetryMode {
    pub fn to_retry_tier(self) -> crate::http::retry::RetryTier {
        match self {
            Self::Strict => crate::http::retry::RetryTier::Strict,
            Self::Standard => crate::http::retry::RetryTier::Standard,
            Self::Bulk => crate::http::retry::RetryTier::Bulk,
        }
    }
}

/// A tool specification that defines a PlayFab API operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolSpec {
    /// Unique tool name (snake_case), e.g. "search_items"
    pub name: String,
    /// Human-readable description
    pub description: String,
    /// PlayFab API group, e.g. "Catalog" or "Admin"
    pub api_group: String,
    /// PlayFab API method name, e.g. "SearchItems"
    pub api_method: String,
    /// Tool category
    pub category: ToolCategory,
    /// Authentication mode
    pub auth_mode: AuthMode,
    /// Retry mode
    pub retry_mode: RetryMode,
    /// JSON Schema for input parameters
    pub input_schema: Value,
}

impl ToolSpec {
    pub fn api_path(&self) -> String {
        format!("{}/{}", self.api_group, self.api_method)
    }
}

/// Build the authentication header pair for a tool's auth mode.
///
/// Returns `(header_name, header_value)` to attach to the HTTP request.
/// For `AuthMode::EntityToken`, the caller must supply a pre-fetched entity
/// token via `entity_token`.
pub fn build_auth_header<'a>(
    auth_mode: AuthMode,
    secret_key: &'a str,
    entity_token: Option<&'a str>,
) -> Result<(&'static str, &'a str)> {
    match auth_mode {
        AuthMode::SecretKey => Ok(("X-SecretKey", secret_key)),
        AuthMode::EntityToken => {
            let token = entity_token.ok_or_else(|| {
                anyhow::anyhow!("Entity token is required for EntityToken auth mode")
            })?;
            Ok(("X-EntityToken", token))
        }
        AuthMode::None => Ok(("X-Placeholder", "")),
    }
}

/// Look up a tool by name from a list of specs
pub fn find_tool<'a>(tools: &'a [ToolSpec], name: &str) -> Option<&'a ToolSpec> {
    tools.iter().find(|t| t.name == name)
}

// ---------------------------------------------------------------------------
// execute_tool — requires PlayFabClient (Unit 2) and AuthManager (Unit 1).
// The full implementation is provided below and will compile once those
// modules expose their public types.
// ---------------------------------------------------------------------------

// pub fn execute_tool(
//     tool: &ToolSpec,
//     params: Value,
//     config: &crate::core::config::PlayFabConfig,
//     client: &crate::http::client::PlayFabClient,
//     auth: &crate::http::auth::AuthManager,
// ) -> Result<Value> {
//     let url = config.api_url(&tool.api_group, &tool.api_method);
//     let tier = tool.retry_mode.to_retry_tier();
//
//     match tool.auth_mode {
//         AuthMode::SecretKey => {
//             client.call_raw(&url, &params, ("X-SecretKey", &config.dev_secret_key), tier)
//         }
//         AuthMode::EntityToken => {
//             let token = auth.get_entity_token()?;
//             client.call_raw(&url, &params, ("X-EntityToken", &token), tier)
//         }
//         AuthMode::None => {
//             client.call_raw(&url, &params, ("X-Placeholder", ""), tier)
//         }
//     }
// }

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn sample_tool() -> ToolSpec {
        ToolSpec {
            name: "search_items".to_string(),
            description: "Search catalog items".to_string(),
            api_group: "Catalog".to_string(),
            api_method: "SearchItems".to_string(),
            category: ToolCategory::EconomyCatalog,
            auth_mode: AuthMode::EntityToken,
            retry_mode: RetryMode::Standard,
            input_schema: json!({"type": "object"}),
        }
    }

    #[test]
    fn api_path_formats_correctly() {
        let tool = sample_tool();
        assert_eq!(tool.api_path(), "Catalog/SearchItems");
    }

    #[test]
    fn find_tool_returns_matching_tool() {
        let tools = vec![sample_tool()];
        let found = find_tool(&tools, "search_items");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "search_items");
    }

    #[test]
    fn find_tool_returns_none_for_unknown() {
        let tools = vec![sample_tool()];
        assert!(find_tool(&tools, "nonexistent").is_none());
    }

    #[test]
    fn retry_mode_maps_to_tier() {
        use crate::http::retry::RetryTier;

        assert_eq!(RetryMode::Strict.to_retry_tier(), RetryTier::Strict);
        assert_eq!(RetryMode::Standard.to_retry_tier(), RetryTier::Standard);
        assert_eq!(RetryMode::Bulk.to_retry_tier(), RetryTier::Bulk);
    }

    #[test]
    fn tool_category_serde_roundtrip() {
        let cat = ToolCategory::EconomyCatalog;
        let json = serde_json::to_string(&cat).unwrap();
        assert_eq!(json, "\"economy_catalog\"");
        let back: ToolCategory = serde_json::from_str(&json).unwrap();
        assert_eq!(back, cat);
    }

    #[test]
    fn auth_mode_serde_roundtrip() {
        let mode = AuthMode::SecretKey;
        let json = serde_json::to_string(&mode).unwrap();
        assert_eq!(json, "\"secret_key\"");
        let back: AuthMode = serde_json::from_str(&json).unwrap();
        assert_eq!(back, mode);
    }

    #[test]
    fn tool_spec_serialization() {
        let tool = sample_tool();
        let json = serde_json::to_value(&tool).unwrap();
        assert_eq!(json["name"], "search_items");
        assert_eq!(json["category"], "economy_catalog");
        assert_eq!(json["auth_mode"], "entity_token");
        assert_eq!(json["retry_mode"], "standard");
    }

    #[test]
    fn build_auth_header_secret_key() {
        let (name, value) = build_auth_header(AuthMode::SecretKey, "my_secret", None).unwrap();
        assert_eq!(name, "X-SecretKey");
        assert_eq!(value, "my_secret");
    }

    #[test]
    fn build_auth_header_entity_token() {
        let (name, value) =
            build_auth_header(AuthMode::EntityToken, "", Some("tok123")).unwrap();
        assert_eq!(name, "X-EntityToken");
        assert_eq!(value, "tok123");
    }

    #[test]
    fn build_auth_header_entity_token_missing() {
        let result = build_auth_header(AuthMode::EntityToken, "", None);
        assert!(result.is_err());
    }

    #[test]
    fn build_auth_header_none() {
        let (name, _value) = build_auth_header(AuthMode::None, "", None).unwrap();
        assert_eq!(name, "X-Placeholder");
    }
}
