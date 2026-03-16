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
