//! Supply-chain guards for GitHub Actions workflows (Issue #39).

use std::fs;
use std::path::Path;

const AUTO_MERGE: &str = include_str!("../.github/workflows/auto-merge.yml");

#[test]
fn auto_merge_is_restricted_to_maintainer_prs() {
    assert!(
        AUTO_MERGE.contains("github.event.pull_request.user.login == 'akiojin'"),
        "auto-merge must only run for PRs authored by akiojin"
    );
    assert!(
        AUTO_MERGE.contains("github.event.pull_request.head.repo.full_name == github.repository"),
        "auto-merge must reject fork PRs"
    );
}

#[test]
fn auto_merge_targets_main_and_develop() {
    assert!(AUTO_MERGE.contains("branches: [main, develop]"));
    assert!(AUTO_MERGE.contains("github.event.pull_request.base.ref == 'develop'"));
}

#[test]
fn pull_request_target_workflows_do_not_checkout_head() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join(".github/workflows");
    for entry in fs::read_dir(dir).expect("workflows dir") {
        let path = entry.expect("entry").path();
        let text = fs::read_to_string(&path).expect("read workflow");
        if !text.contains("pull_request_target") {
            continue;
        }
        assert!(
            !text.contains("actions/checkout"),
            "{} uses pull_request_target and must not check out PR code",
            path.display()
        );
        assert!(
            text.contains("github.event.pull_request.user.login == 'akiojin'"),
            "{} uses pull_request_target and must be restricted to akiojin PRs",
            path.display()
        );
    }
}

#[test]
fn no_other_workflow_auto_merges_or_approves() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join(".github/workflows");
    for entry in fs::read_dir(dir).expect("workflows dir") {
        let path = entry.expect("entry").path();
        if path.file_name().is_some_and(|n| n == "auto-merge.yml") {
            continue;
        }
        let text = fs::read_to_string(&path).expect("read workflow");
        for needle in [
            "gh pr merge",
            "gh pr review",
            "--approve",
            "enable-automerge",
        ] {
            assert!(
                !text.contains(needle),
                "{} contains `{needle}`; auto-merge/approval must stay in auto-merge.yml",
                path.display()
            );
        }
    }
}
