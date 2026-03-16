# Contributing to playfab-cli

English | [日本語](#日本語)

Thanks for contributing to `playfab-cli`.

## Prerequisites

- Rust stable
- git-cliff (`cargo install git-cliff`)
- gh CLI (for release automation)

## Development Setup

```bash
git clone https://github.com/akiojin/playfab-cli.git
cd playfab-cli
cargo build
```

### Docker (Optional)

You can use Docker without installing Rust locally.

```bash
docker build -t playfab-cli-dev .
docker run --rm playfab-cli-dev
```

## Validation Commands

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

### Running Tests

```bash
cargo test
```

## Branch Policy

- Default target branch: `develop`
- `main` accepts only release PRs from `develop` or release automation branches

## Commit Style

Use Conventional Commits:

- `feat: ...`
- `fix: ...`
- `chore: ...`
- `docs: ...`
- `test: ...`
- `refactor: ...`
- `perf: ...`

## TDD

Follow RED -> GREEN -> REFACTOR.
Add/adjust tests in the same change set as implementation.

## Release

Releases are managed via the `/release` Claude command or `scripts/publish.sh`:

```bash
./scripts/publish.sh <major|minor|patch>
```

## License and Attribution

`playfab-cli` is MIT licensed. MIT requires preserving the copyright + permission notice.

---

## 日本語

`playfab-cli` へのコントリビュートありがとうございます。

## 前提ツール

- Rust stable
- git-cliff（`cargo install git-cliff`）
- gh CLI（リリース自動化用）

## セットアップ

```bash
git clone https://github.com/akiojin/playfab-cli.git
cd playfab-cli
cargo build
```

### Docker（任意）

ローカルに Rust をインストールせずに Docker で検証できます。

```bash
docker build -t playfab-cli-dev .
docker run --rm playfab-cli-dev
```

## 検証コマンド

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

### テスト実行

```bash
cargo test
```

## ブランチ運用

- 通常のPR先は `develop`
- `main` へのPRはリリース系のみ

## コミット規約

Conventional Commits を使用してください（`feat:`, `fix:`, `chore:`, `docs:`, `test:`, `refactor:`, `perf:` など）。

## TDD

RED -> GREEN -> REFACTOR を前提に進めてください。実装変更には対応テストを含めます。

## リリース

リリースは `/release` Claude コマンドまたは `scripts/publish.sh` で管理します：

```bash
./scripts/publish.sh <major|minor|patch>
```

## ライセンスと表記

`playfab-cli` は MIT ライセンスです。MIT条項に従い、著作権表示と許諾表示を保持してください。
