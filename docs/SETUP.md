# セットアップガイド

このプロジェクトのセットアップ手順を説明します。

## 前提条件

- Rust (latest stable)
- Cargo
- Bun (Package Manager)
- Docker (Optional, for DB if needed later)

## 環境構築手順

### 1. リポジトリのクローン

```bash
git clone https://github.com/otomatty/continuum.git
cd continuum
```

### 2. 環境変数の設定

`.env.example` をコピーして `.env` ファイルを作成し、必要な値を設定します。

```bash
cp .env.example .env
```

`.env` ファイルを編集し、以下の値を設定してください：

- `GITHUB_CLIENT_ID`: GitHub OAuth AppのClient ID
- `GITHUB_CLIENT_SECRET`: GitHub OAuth AppのClient Secret
- `GITHUB_ORG_NAME`: 対象のGitHub Organization名
- `SESSION_SECRET`: セッション暗号化用のランダム文字列（`openssl rand -base64 32` 等で生成）

### 3. 依存関係のインストール

```bash
bun install
```

### 4. 開発サーバーの起動

```bash
cargo leptos watch
```

## GitHub OAuth Appの作成方法

1. GitHubにログインし、[Developer settings > OAuth Apps](https://github.com/settings/developers) にアクセスします。
2. "New OAuth App" をクリックします。
3. 以下の情報を入力します：
   - **Application name**: Continuum (任意)
   - **Homepage URL**: `http://localhost:3000`
   - **Authorization callback URL**: `http://localhost:3000/auth/callback`
4. "Register application" をクリックします。
5. 生成された **Client ID** と **Client Secret** を `.env` ファイルにコピーします。

## トラブルシューティング

### 環境変数が読み込まれない

- `.env` ファイルがプロジェクトルート（`Cargo.toml` と同じ階層）にあるか確認してください。
- サーバーを再起動してください。

### ビルドエラー

- Rustのバージョンが古い可能性があります。`rustup update` を実行してください。
- `cargo-leptos` がインストールされているか確認してください：`cargo install cargo-leptos`

