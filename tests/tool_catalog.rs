// Test the tool catalog
use std::collections::HashSet;
use std::process::Command;

fn cli_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_playfab-cli"))
}

#[test]
fn all_tools_returns_non_empty() {
    let output = cli_bin()
        .args(["--output", "json", "tool", "list"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let tools: Vec<String> = serde_json::from_str(&stdout).expect("valid JSON array");
    assert!(!tools.is_empty(), "tool catalog should not be empty");
    assert!(tools.len() > 50, "expected 50+ tools, got {}", tools.len());
}

#[test]
fn no_duplicate_tool_names() {
    let output = cli_bin()
        .args(["--output", "json", "tool", "list"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let tools: Vec<String> = serde_json::from_str(&stdout).expect("valid JSON");
    let unique: HashSet<&str> = tools.iter().map(|s| s.as_str()).collect();
    assert_eq!(tools.len(), unique.len(), "duplicate tool names found");
}

#[test]
fn all_tools_have_valid_schema() {
    let output = cli_bin()
        .args(["--output", "json", "tool", "schema"])
        .output()
        .expect("failed to execute");
    assert!(
        output.status.success(),
        "tool schema failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let schemas: Vec<serde_json::Value> = serde_json::from_str(&stdout).expect("valid JSON");
    for schema in &schemas {
        assert!(schema["name"].is_string(), "tool must have name");
        assert!(
            schema["description"].is_string(),
            "tool must have description"
        );
    }
}

fn tool_schema(name: &str) -> serde_json::Value {
    let output = cli_bin()
        .args(["--output", "json", "tool", "schema", name])
        .output()
        .expect("failed to execute");
    assert!(
        output.status.success(),
        "tool schema {name} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("valid JSON")
}

fn sorted_strings(value: &serde_json::Value) -> Vec<String> {
    let mut items: Vec<String> = value
        .as_array()
        .expect("array")
        .iter()
        .map(|v| v.as_str().expect("string").to_string())
        .collect();
    items.sort();
    items
}

fn assert_tool_contract(
    name: &str,
    api_path: (&str, &str),
    auth_mode: &str,
    properties: &[&str],
    required: &[&str],
) {
    let schema = tool_schema(name);
    assert_eq!(schema["api_group"], api_path.0, "{name} api_group");
    assert_eq!(schema["api_method"], api_path.1, "{name} api_method");
    assert_eq!(schema["auth_mode"], auth_mode, "{name} auth_mode");

    let mut actual_props: Vec<String> = schema["input_schema"]["properties"]
        .as_object()
        .expect("properties object")
        .keys()
        .cloned()
        .collect();
    actual_props.sort();
    let mut expected_props: Vec<String> = properties.iter().map(|s| s.to_string()).collect();
    expected_props.sort();
    assert_eq!(actual_props, expected_props, "{name} properties");

    let mut expected_required: Vec<String> = required.iter().map(|s| s.to_string()).collect();
    expected_required.sort();
    assert_eq!(
        sorted_strings(&schema["input_schema"]["required"]),
        expected_required,
        "{name} required"
    );
}

#[test]
fn admin_add_news_matches_official_reference() {
    assert_tool_contract(
        "admin_add_news",
        ("Admin", "AddNews"),
        "secret_key",
        &["Body", "Title", "CustomTags", "Status", "Timestamp"],
        &["Body", "Title"],
    );
    let schema = tool_schema("admin_add_news");
    assert_eq!(
        sorted_strings(&schema["input_schema"]["properties"]["Status"]["enum"]),
        vec!["Archived", "None", "Published", "Unpublished"]
    );
}

#[test]
fn admin_add_localized_news_matches_official_reference() {
    assert_tool_contract(
        "admin_add_localized_news",
        ("Admin", "AddLocalizedNews"),
        "secret_key",
        &["Body", "Language", "NewsId", "Title", "CustomTags"],
        &["Body", "Language", "NewsId", "Title"],
    );
}

#[test]
fn admin_revoke_all_bans_for_user_matches_official_reference() {
    assert_tool_contract(
        "admin_revoke_all_bans_for_user",
        ("Admin", "RevokeAllBansForUser"),
        "secret_key",
        &["PlayFabId"],
        &["PlayFabId"],
    );
}

#[test]
fn get_title_players_from_master_player_account_ids_matches_official_reference() {
    assert_tool_contract(
        "get_title_players_from_master_player_account_ids",
        ("Profile", "GetTitlePlayersFromMasterPlayerAccountIds"),
        "entity_token",
        &["MasterPlayerAccountIds", "TitleId", "CustomTags"],
        &["MasterPlayerAccountIds"],
    );
    let schema = tool_schema("get_title_players_from_master_player_account_ids");
    assert_eq!(
        schema["input_schema"]["properties"]["MasterPlayerAccountIds"]["type"],
        "array"
    );
}

#[test]
fn nonexistent_get_title_players_account_api_is_not_exposed() {
    let output = cli_bin()
        .args(["--output", "json", "tool", "schema"])
        .output()
        .expect("failed to execute");
    let schemas: Vec<serde_json::Value> =
        serde_json::from_slice(&output.stdout).expect("valid JSON");
    assert!(
        schemas
            .iter()
            .all(|s| s["name"] != "get_title_players_account"
                && s["api_method"] != "GetTitlePlayersAccount"),
        "Profile/GetTitlePlayersAccount does not exist in the official reference"
    );
}
