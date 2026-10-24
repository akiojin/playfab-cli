// Validate skill definitions against the tool catalog and their copies
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Canonical skill directory. `.claude/skills/` and `.codex/skills/` are copies.
const CANONICAL_SKILLS_DIR: &str = ".claude-plugin/plugins/playfab-cli/skills";
const SKILL_COPY_DIRS: &[&str] = &[".claude/skills", ".codex/skills"];

const REQUIRED_SECTIONS: &[&str] = &[
    "Use When",
    "Do Not Use When",
    "Available Tools",
    "Examples",
    "Common Workflows",
    "Notes",
];

/// Skills that intentionally have no `Available Tools` section.
/// `usage` documents CLI infrastructure commands, not catalog tools.
const SKILLS_WITHOUT_AVAILABLE_TOOLS: &[&str] = &["usage"];

/// Backticked words that may appear in an `Available Tools` section but are not tool names
/// (e.g. entity type names).
const NON_TOOL_TERMS: &[&str] = &[
    "title",
    "master_player_account",
    "title_player_account",
    "character",
    "group",
    "game_server",
    "namespace",
    "service",
];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn catalog_tools() -> BTreeSet<String> {
    let output = Command::new(env!("CARGO_BIN_EXE_playfab-cli"))
        .args(["--output", "json", "tool", "list"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success(), "tool list failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str::<Vec<String>>(&stdout)
        .expect("valid JSON array")
        .into_iter()
        .collect()
}

/// Returns `(skill name, SKILL.md content)` for every canonical skill.
fn canonical_skills() -> BTreeMap<String, String> {
    let dir = repo_root().join(CANONICAL_SKILLS_DIR);
    let mut skills = BTreeMap::new();
    for entry in fs::read_dir(&dir).expect("canonical skills dir exists") {
        let path = entry.expect("readable dir entry").path();
        if !path.is_dir() {
            continue;
        }
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        let content = fs::read_to_string(path.join("SKILL.md"))
            .unwrap_or_else(|e| panic!("{name}/SKILL.md must be readable: {e}"));
        skills.insert(name, content);
    }
    assert!(
        !skills.is_empty(),
        "no skills found in {CANONICAL_SKILLS_DIR}"
    );
    skills
}

/// Body of the `## <title>` section up to the next `## ` heading.
fn section<'a>(content: &'a str, title: &str) -> Option<&'a str> {
    let heading = format!("## {title}");
    let mut start = None;
    let mut offset = 0;
    for line in content.split_inclusive('\n') {
        let trimmed = line.trim_end();
        match start {
            None if trimmed == heading => start = Some(offset + line.len()),
            Some(s) if trimmed.starts_with("## ") => return Some(&content[s..offset]),
            _ => {}
        }
        offset += line.len();
    }
    start.map(|s| &content[s..])
}

/// Tool-like identifiers written in backticks inside `Available Tools`.
fn listed_tools(content: &str) -> BTreeSet<String> {
    let Some(body) = section(content, "Available Tools") else {
        return BTreeSet::new();
    };
    body.split('`')
        .skip(1)
        .step_by(2)
        .filter(|word| {
            !word.is_empty()
                && word.starts_with(|c: char| c.is_ascii_lowercase())
                && word
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        })
        .filter(|word| !NON_TOOL_TERMS.contains(word))
        .map(str::to_string)
        .collect()
}

fn frontmatter(content: &str) -> Option<&str> {
    let rest = content.strip_prefix("---\n")?;
    let end = rest.find("\n---\n")?;
    Some(&rest[..end])
}

fn frontmatter_value<'a>(frontmatter: &'a str, key: &str) -> Option<&'a str> {
    frontmatter.lines().find_map(|line| {
        line.strip_prefix(key)
            .and_then(|rest| rest.strip_prefix(':'))
            .map(str::trim)
    })
}

fn collect_files(root: &Path, dir: &Path, files: &mut BTreeMap<PathBuf, Vec<u8>>) {
    for entry in fs::read_dir(dir).expect("readable dir") {
        let path = entry.expect("readable dir entry").path();
        if path.is_dir() {
            collect_files(root, &path, files);
        } else {
            let relative = path.strip_prefix(root).unwrap().to_path_buf();
            files.insert(relative, fs::read(&path).expect("readable file"));
        }
    }
}

#[test]
fn skill_tools_exist_in_catalog() {
    let tools = catalog_tools();
    let mut unknown = Vec::new();
    for (skill, content) in canonical_skills() {
        for tool in listed_tools(&content) {
            if !tools.contains(&tool) {
                unknown.push(format!("{skill}: {tool}"));
            }
        }
    }
    assert!(
        unknown.is_empty(),
        "skills list tools missing from src/tooling/catalog (add to NON_TOOL_TERMS if not a tool):\n{}",
        unknown.join("\n")
    );
}

#[test]
fn every_catalog_tool_is_covered_by_a_skill() {
    let listed: BTreeSet<String> = canonical_skills()
        .values()
        .flat_map(|content| listed_tools(content))
        .collect();
    let uncovered: Vec<String> = catalog_tools().difference(&listed).cloned().collect();
    assert!(
        uncovered.is_empty(),
        "catalog tools not listed in any skill's Available Tools:\n{}",
        uncovered.join("\n")
    );
}

#[test]
fn skill_copies_match_canonical() {
    let root = repo_root();
    for skill in canonical_skills().keys() {
        let canonical_dir = root.join(CANONICAL_SKILLS_DIR).join(skill);
        let mut canonical = BTreeMap::new();
        collect_files(&canonical_dir, &canonical_dir, &mut canonical);
        for copy in SKILL_COPY_DIRS {
            let copy_dir = root.join(copy).join(skill);
            assert!(copy_dir.is_dir(), "{copy}/{skill} is missing");
            let mut copied = BTreeMap::new();
            collect_files(&copy_dir, &copy_dir, &mut copied);
            assert_eq!(
                canonical.keys().collect::<Vec<_>>(),
                copied.keys().collect::<Vec<_>>(),
                "{copy}/{skill} has a different file set than the canonical skill"
            );
            for (file, bytes) in &canonical {
                assert!(
                    copied[file] == *bytes,
                    "{copy}/{skill}/{} differs from the canonical skill",
                    file.display()
                );
            }
        }
    }
}

#[test]
fn skills_have_frontmatter_and_required_sections() {
    for (skill, content) in canonical_skills() {
        let content = content.replace("\r\n", "\n");
        let fm =
            frontmatter(&content).unwrap_or_else(|| panic!("{skill}: missing YAML frontmatter"));
        assert_eq!(
            frontmatter_value(fm, "name"),
            Some(skill.as_str()),
            "{skill}: frontmatter name must match the directory name"
        );
        assert!(
            frontmatter_value(fm, "description").is_some_and(|d| !d.is_empty()),
            "{skill}: frontmatter description is required"
        );
        for required in REQUIRED_SECTIONS {
            if *required == "Available Tools" && SKILLS_WITHOUT_AVAILABLE_TOOLS.contains(&&*skill) {
                continue;
            }
            assert!(
                section(&content, required).is_some(),
                "{skill}: missing required section `## {required}`"
            );
        }
    }
}

#[test]
fn listed_tools_ignores_non_tool_terms_and_other_sections() {
    let content = "## Available Tools\n\n| `get_item` | Uses `title_player_account` and `Foo` |\n\n## Examples\n\n`delete_item`\n";
    let tools: Vec<String> = listed_tools(content).into_iter().collect();
    assert_eq!(tools, vec!["get_item".to_string()]);
}
