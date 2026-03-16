# PlayFab CLI プロジェクトガイドライン

## 重要なルール

- **日本語で回答する**
- **エコノミーAPIはv2のみを利用する** - レガシーv1 APIは使用しない
- **ソースコード修正後は必ずビルドする** - `cargo build` を実行
- **コミットログはcommitlint形式で生成する** - `type(scope): description` の形式
- **コミットログは日本語** - コミットメッセージの説明部分は日本語で記載
- **コミットの際には、必ず承認を取ること** - ユーザーの明示的な許可なしにコミットしない
- **PlayFab REST APIの追加・修正時には必ず公式サイトでパラメーターの整合性を取ること** - https://learn.microsoft.com/ja-jp/rest/api/playfab/?view=playfab-rest
- **CHANGELOG.mdは英語で記載する**
- **ビルドエラーは必ず排除する** - コンパイルエラーを放置しない

## 概要

PlayFab サービスを自動化するための Rust CLI ツールです。200以上のツール定義を持ち、PlayFab API を網羅的にカバーします。

## 環境設定

### 必要な環境変数

```bash
PLAYFAB_TITLE_ID=your_title_id           # PlayFabタイトルID
PLAYFAB_DEV_SECRET_KEY=your_secret_key   # PlayFab開発者シークレットキー
```

## 開発コマンド

```bash
# ビルド
cargo build
cargo build --release

# テスト
cargo test

# フォーマットチェック
cargo fmt --check

# Lintチェック
cargo clippy -- -D warnings

# 実行
cargo run -- tool list
cargo run -- system ping
```

## 品質ゲート

コミット前に以下を全てパスさせること:

1. `cargo fmt --check` - フォーマット
2. `cargo clippy -- -D warnings` - Lint
3. `cargo test` - テスト
4. `cargo build` - ビルド成功

## プロジェクト構造

```text
src/
├── main.rs              # エントリーポイント
├── cli.rs               # CLI引数定義 (clap)
├── app/                 # アプリケーション層
│   ├── mod.rs
│   └── runner.rs        # コマンドディスパッチ
├── core/                # コア機能
│   ├── config.rs        # 設定管理
│   ├── self_update.rs   # 自己更新
│   └── managed_binaries.rs
├── http/                # HTTP通信層
│   ├── client.rs        # HTTPクライアント
│   ├── auth.rs          # Entity Token認証
│   └── retry.rs         # リトライ制御
└── tooling/             # ツール定義・実行
    ├── tool_executor.rs # ツール実行エンジン
    ├── schema_builder.rs # JSONスキーマ構築
    └── catalog/         # カテゴリ別ツール定義
```

## API実装規則

1. ツール定義は `src/tooling/catalog/` 配下に作成
2. Economy関連はすべてv2 APIを使用
3. HTTPクライアントは `src/http/client.rs` のラッパーを使用
4. 認証は Entity Token ベース (`src/http/auth.rs`)
5. リトライはティア別に制御 (Strict/Standard/Bulk)

## スキルアーキテクチャ

旧 `playfab-mcp-server` の 31 個の MCP ツールを **Claude Code Skill** に変換。
playfab-cli の 413 ツールを 15 個のスキルでカバー。
スキルはオンデマンドで読み込まれ、LLM コンテキストを肥大化させない。
内部的には `playfab-cli tool call <name>` コマンドを呼び出す。

- スキル定義: `.claude-plugin/plugins/playfab-cli/skills/`
- プラグインマニフェスト: `.claude-plugin/plugins/playfab-cli/plugin.json`
- Claude Code テスト登録: `.claude/skills/`（正本からのコピー）
- Codex 運用: `.codex/skills/`（正本からのコピー）
- 旧MCP由来のスキル名/互換エイリアスは提供しない

### スキル一覧

| スキル | カテゴリ | ツール数 | ソース |
|--------|----------|---------|--------|
| usage | foundation | N/A | CLI インフラ |
| catalog | economy | 26 | `economy_catalog.rs` |
| inventory | economy | 20 | `economy_inventory.rs` |
| player | entity | 19 | `profiles.rs` + `data.rs` + `authentication.rs` |
| title | config | 4 | Economy config サブセット |
| bulk | operations | N/A | ワークフロースキル |
| economy | economy | N/A | catalog/inventory クロスカット |
| leaderboards | progression | 24 | `progression.rs` |
| multiplayer | multiplayer | 68 | `multiplayer.rs` |
| cloudscript | scripting | 14 | `cloudscript.rs` |
| events | analytics | 7 | `events.rs` |
| groups | entity | 25 | `groups.rs` |
| admin | admin | 84 | `admin.rs` |
| server | server | 113 | `server.rs` |
| experimentation | analytics | 13 | `experimentation.rs` |

### SKILL.md フォーマット

各スキルは `skills/<name>/SKILL.md` 形式で、以下の構造を持つ:

```yaml
---
name: <skill-name>
description: Use when... Do not use when...
allowed-tools: Bash, Read, Grep, Glob
metadata:
  author: akiojin
  version: 0.2.0
  category: <category>
---
```

必須セクション: Use When, Do Not Use When, Available Tools, Examples, Common Workflows, Notes

## Claude Code 運用ワークフロー

### 1. Plan Mode を既定にする

- 3ステップ以上、または設計判断を含む作業は Plan Mode で開始する
- 実装だけでなく、検証・ロールバック方針も計画に含める
- 途中で前提が崩れたら実装を止めて再計画する
- 新規機能・大きな変更は `gwt-spec` ラベル付き GitHub Issue の `Spec` / `Plan` / `Tasks` / `TDD` を先に更新する

### 2. サブエージェントを意図的に使う

- 調査、ログ解析、差分比較、長時間テストはサブエージェントへ委譲する
- 1サブエージェントにつき1タスクを原則とする
- メインスレッドは意思決定と統合に集中し、コンテキスト汚染を防ぐ

### 3. 完了前に必ず検証する

- 動作証明なしで「完了」としない
- テスト実行、ログ確認、必要時の `main` 比較を行う
- 「スタッフエンジニアがレビューで承認できるか」を自己チェックする

### 4. エレガントさを追求する（過剰設計しない）

- 非自明な変更は「よりシンプルで堅牢な解があるか」を一度見直す
- ハック的修正は避け、根本原因に対する実装を優先する
- 小さく明白な修正では速度を優先し、不要な抽象化を入れない
