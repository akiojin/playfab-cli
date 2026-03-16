# PlayFab CLI 進捗管理

## プロジェクト概要

PlayFab MCP サーバー（TypeScript、31ツール）を置き換える Rust CLI ツール。
413ツールを13カテゴリに分類し、Claude Code Skills によるLLM統合を実現。

- **言語**: Rust
- **ツール数**: 413（13カテゴリ）
- **LLM統合**: Claude Code Skills（MCP ではなく CLI + Skills 方式）
- **前身**: playfab-mcp-server（TypeScript MCP サーバー）

## コミット履歴

| ハッシュ | 内容 |
|---------|------|
| `d8dfc4d` | chore: スケルトン構造の初回コミット |
| `e3531f9` | feat(cli): PlayFab CLI v0.1.0 全実装 |
| `e36dd6b` | feat(skills): スキルをディレクトリ形式に変換し、Admin/Server/Experimentation スキルを新規追加 |
| `8b68c2c` | docs(plans): 作業履歴と今後のタスクを .agent/PLANS.md に記録 |
| `e5299d9` | feat(tooling): ツール実行エンジンの完成と clippy 修正 |
| `f6a917a` | chore(config): プロジェクト設定ファイルとドキュメントを追加 |
| `e8bde2b` | chore(claude): Claude Code 設定、エージェント、コマンドを追加 |

## 完了タスク

### v0.1.0 — Rust CLI 基盤構築

- [x] Rust CLI スケルトン構造の作成
- [x] PlayFab CLI v0.1.0 全実装（413ツール、13カテゴリ）
  - Economy v2 Catalog (26ツール)
  - Economy v2 Inventory (20ツール)
  - Admin API (84ツール)
  - Server API (113ツール)
  - Authentication (2ツール)
  - Profiles (10ツール)
  - Data (7ツール)
  - CloudScript (14ツール)
  - Events (7ツール)
  - Groups (25ツール)
  - Multiplayer (68ツール)
  - Progression (24ツール)
  - Experimentation (13ツール)
- [x] HTTP クライアント実装（リトライティア: Strict/Standard/Bulk）
- [x] Entity Token 認証（自動キャッシュ＆リフレッシュ）
- [x] CLI コマンド実装
  - `tool list` / `tool schema` / `tool call`
  - `system ping`
  - `config show` / `config set`
  - `batch`
  - `cli install` / `cli doctor`
- [x] JSON/Text 出力フォーマット対応
- [x] Self-update via GitHub Releases
- [x] CI/CD with cross-compilation for 4 targets

### v0.2.0 — Claude Code Skills 整備

- [x] Claude Code plugin 作成（12 → 15 ドメインスキル）
- [x] スキルをディレクトリ形式 (`skills/foo/SKILL.md`) に変換
- [x] YAML frontmatter を全スキルに追加
- [x] Use When / Do Not Use When / Notes セクションを全スキルに追加
- [x] Admin API スキル新規作成（84ツール）
- [x] Server API スキル新規作成（113ツール）
- [x] Experimentation API スキル新規作成（13ツール）
- [x] `plugin.json` を v0.2.0 に更新（15スキル登録）
- [x] `.claude/skills/` と `.codex/skills/` を正本と同期（各15スキル）
- [x] CLAUDE.md にスキルアーキテクチャセクションと運用ワークフローを追記
- [x] 全413ツールの100%スキルカバレッジ達成

## 進行中のタスク

### 現在作業中

（なし）

### v0.1.1 — ツール実行エンジン完成＆プロジェクト設定

- [x] tool_executor の execute_tool 実装（スタブからAPI呼び出しに置換）
- [x] catalog/mod.rs の重複 ToolSpec 定義を削除し tool_executor から re-export
- [x] clippy 警告の修正（range contains, iterator enumerate, is_multiple_of 等）
- [x] 全13カテゴリのツール定義に api_group/api_method/auth_mode/retry_mode を追加
- [x] プロジェクト設定ファイル追加（.editorconfig, Dockerfile, CI設定, cliff.toml 等）
- [x] CONTRIBUTING.md, SECURITY.md, AGENTS.md 追加
- [x] Claude Code 設定・エージェント・コマンド追加

## 今後のタスク

### 優先度: 高（1〜2週間以内）

- [ ] 未コミットの Rust ソース変更をレビューしてコミット（Cargo.toml, src/ 配下の全変更）
- [ ] 未トラックファイルの整理とコミット（.editorconfig, .github/, Dockerfile, scripts/ 等）
- [ ] playfab-mcp-server の非推奨化宣言（README に非推奨通知、playfab-cli への移行案内を追加）
- [ ] `cargo build` / `cargo test` の CI パス確認
- [ ] `cargo fmt --check` / `cargo clippy -- -D warnings` のパス確認

### 優先度: 中（1ヶ月以内）

- [ ] GitHub Actions ワークフローの整備（テスト、リント、ビルド、リリース）
- [ ] テストの実装（現在テストなし）
- [ ] エラーハンドリングの改善（HTTP レスポンスのエラー処理）
- [ ] レート制限対策の実装（自動リトライ、バックオフ）
- [ ] バージョニング戦略の確定（Cargo.toml version と CHANGELOG.md の同期）

### 優先度: 低（3ヶ月以内）

- [ ] crates.io への公開準備
- [ ] ドキュメントの充実（README に詳細な使用例を追加）
- [ ] GitHub Releases の自動化（cliff.toml によるリリースノート生成）
- [ ] Docker イメージの公開
- [ ] プラットフォーム別バイナリの配布（homebrew, scoop 等）

### 技術的負債の解消

- [ ] Rust コードの `unwrap()` / `expect()` を適切なエラーハンドリングに置換
- [ ] 各モジュールのドキュメントコメント（`///` doc comments）追加
- [ ] `cargo doc` によるAPIドキュメント生成
- [ ] ベンチマークテストの追加（criterion クレート）
- [ ] 依存クレートのセキュリティ監査（`cargo audit`）

### 将来の拡張機能

- [ ] PlayFab 以外のゲームサービス対応の検討
- [ ] WebSocket/リアルタイム通信サポート
- [ ] プラグインシステムの導入
- [ ] TUI（ターミナルUI）モードの追加
- [ ] コミュニティプラグインのエコシステム構築
