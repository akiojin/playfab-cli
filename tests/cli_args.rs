use std::process::Command;

fn cli_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_playfab-cli"))
}

#[test]
fn no_args_shows_help() {
    let output = cli_bin().output().expect("failed to execute");
    assert!(!output.status.success()); // arg_required_else_help
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let combined = format!("{}{}", stdout, stderr);
    assert!(
        combined.contains("Usage") || combined.contains("playfab-cli"),
        "expected help output, got stdout={stdout}, stderr={stderr}"
    );
}

#[test]
fn tool_list_succeeds() {
    let output = cli_bin()
        .args(["tool", "list"])
        .output()
        .expect("failed to execute");
    assert!(
        output.status.success(),
        "tool list failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn tool_list_json_output() {
    let output = cli_bin()
        .args(["--output", "json", "tool", "list"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    assert!(parsed.is_array());
}

#[test]
fn tool_schema_for_specific_tool() {
    let output = cli_bin()
        .args(["--output", "json", "tool", "schema", "search_items"])
        .output()
        .expect("failed to execute");
    assert!(
        output.status.success(),
        "tool schema failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    assert_eq!(parsed["name"], "search_items");
}

#[test]
fn tool_schema_unknown_tool_fails() {
    let output = cli_bin()
        .args(["tool", "schema", "nonexistent_tool"])
        .output()
        .expect("failed to execute");
    assert!(!output.status.success());
}

#[test]
fn system_ping_without_env_fails_gracefully() {
    let output = cli_bin()
        .args(["system", "ping"])
        .env_remove("PLAYFAB_TITLE_ID")
        .env_remove("PLAYFAB_DEV_SECRET_KEY")
        .output()
        .expect("failed to execute");
    // system ping requires PlayFabConfig::from_env() which needs env vars
    assert!(!output.status.success());
}

#[test]
fn cli_doctor_works() {
    let output = cli_bin()
        .args(["cli", "doctor"])
        .output()
        .expect("failed to execute");
    // Doctor should succeed even without env vars (it reports status)
    assert!(
        output.status.success(),
        "cli doctor failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("version") || stdout.contains("config"),
        "expected doctor output, got: {stdout}"
    );
}

#[test]
fn version_flag() {
    let output = cli_bin()
        .args(["--version"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("playfab-cli"),
        "expected version output containing 'playfab-cli', got: {stdout}"
    );
}
