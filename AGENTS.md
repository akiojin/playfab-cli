# PlayFab CLI — Codex / Agent Workflow

## Shell commands

Always use `bash -lc "cmd"` with `workdir` pointing to the repo root.

## Searching

Prefer `rg` (ripgrep) for file searches. Use `--type rust` for Rust files.

## Editing

Use `apply_patch` for small single-file changes. For multi-file refactors, write each file fully.

## Response style

- Concise, imperative sentences
- Reference files as `path:line`
- Always suggest one improvement at the end
