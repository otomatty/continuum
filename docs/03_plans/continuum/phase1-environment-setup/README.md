# Phase 1-3: 環境変数管理とセットアップ

## 概要

開発環境から本番環境まで、環境変数を適切に管理する仕組みを構築します。

## 目標

- 環境変数の一元管理
- セキュアな設定値の管理
- 開発者向けセットアップ手順の整備

## 実装内容

### 1. ファイル構成

```
.
├── .env.example        # 環境変数テンプレート
├── .env                # ローカル開発用（gitignore）
├── .env.local          # ローカル開発用（gitignore、オプション）
└── .gitignore          # .env を除外
```

### 2. 必要な環境変数

#### 2.1 GitHub OAuth設定

```bash
# GitHub OAuth App設定
GITHUB_CLIENT_ID=your_github_client_id
GITHUB_CLIENT_SECRET=your_github_client_secret
GITHUB_OAUTH_CALLBACK_URL=http://localhost:3000/auth/callback

# GitHub Organization設定
GITHUB_ORG_NAME=your_organization_name
```

#### 2.2 セッション設定

```bash
# セッション暗号化キー（32文字以上のランダム文字列）
SESSION_SECRET=your_random_secret_key_32_chars_minimum
```

#### 2.3 サーバー設定

```bash
# サーバーアドレス
LEPTOS_SITE_ADDR=127.0.0.1:3000

# 環境（DEV/PROD）
ENV=DEV
```

#### 2.4 Cloudflare設定（Phase 3で使用）

```bash
# Cloudflare Workers設定
CLOUDFLARE_ACCOUNT_ID=your_account_id
CLOUDFLARE_API_TOKEN=your_api_token
CLOUDFLARE_KV_NAMESPACE_ID=your_kv_namespace_id
```

### 3. 環境変数読み込み実装

#### 3.1 設定構造体 (`server/src/config.rs`)

```rust
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub github: GitHubConfig,
    pub session: SessionConfig,
    pub server: ServerConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GitHubConfig {
    pub client_id: String,
    pub client_secret: String,
    pub callback_url: String,
    pub org_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SessionConfig {
    pub secret: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub addr: String,
    pub env: String,
}
```

#### 3.2 環境変数読み込み関数

```rust
impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        // dotenvを使用して.envファイルを読み込み
        // 環境変数からConfig構造体を構築
    }
}
```

### 4. 依存関係

```toml
# server/Cargo.toml に追加
[dependencies]
dotenv = "0.15"
serde = { version = "1.0", features = ["derive"] }
```

### 5. `.env.example` テンプレート

```bash
# GitHub OAuth App設定
# GitHub Settings > Developer settings > OAuth Apps で作成
GITHUB_CLIENT_ID=
GITHUB_CLIENT_SECRET=
GITHUB_OAUTH_CALLBACK_URL=http://localhost:3000/auth/callback

# GitHub Organization設定
GITHUB_ORG_NAME=

# セッション暗号化キー（32文字以上のランダム文字列を生成）
# 生成方法: openssl rand -base64 32
SESSION_SECRET=

# サーバー設定
LEPTOS_SITE_ADDR=127.0.0.1:3000
ENV=DEV

# Cloudflare設定（Phase 3で使用）
# CLOUDFLARE_ACCOUNT_ID=
# CLOUDFLARE_API_TOKEN=
# CLOUDFLARE_KV_NAMESPACE_ID=
```

### 6. セットアップ手順ドキュメント

**新規開発者向けセットアップ手順:**

1. リポジトリをクローン
2. `.env.example` を `.env` にコピー
3. GitHub OAuth App作成（手順は別ドキュメント参照）
4. `.env` に必要な値を設定
5. `cargo leptos watch` で起動

### 7. 環境変数検証

**起動時の検証:**
- 必須環境変数の存在チェック
- 値の形式チェック（URL、長さ等）
- エラー時の明確なメッセージ表示

**実装:**
```rust
impl Config {
    pub fn validate(&self) -> Result<(), ValidationError> {
        // 各設定値の検証
    }
}
```

### 8. セキュリティ考慮事項

- `.env` ファイルはgitignoreに追加
- 本番環境では環境変数を直接設定（ファイル不使用）
- シークレット情報はログに出力しない
- `.env.example` には実際の値を含めない

## 実装手順

1. `.env.example` ファイル作成
2. `.gitignore` に `.env` 追加
3. `server/src/config.rs` 作成
4. 環境変数読み込み実装
5. 設定検証機能実装
6. セットアップ手順ドキュメント作成

## 関連ドキュメント

- 親計画: `../20250101_01_setup-plan.md`
- GitHub OAuth設定: `../phase1-github-oauth/README.md`

