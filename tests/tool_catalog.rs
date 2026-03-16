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
