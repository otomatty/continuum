# Phase 3-1: Cloudflare Workers設定

## 概要

AxumバックエンドをCloudflare Workers上で動作させるための設定を行います。

## 目標

- Cloudflare Workers用のビルド設定
- Leptos SSRのWorkers対応
- KV名前空間の設定
- 環境変数の設定

## 実装内容

### 1. ファイル構成

```
.
├── wrangler.toml       # Cloudflare Workers設定
├── .dev.vars           # ローカル開発用環境変数
└── server/
    └── src/
        └── worker.rs   # Workers用エントリーポイント
```

### 2. wrangler.toml設定

```toml
name = "continuum-api"
main = "target/server/release/server.js"
compatibility_date = "2024-01-01"

[env.production]
name = "continuum-api"
route = { pattern = "api.continuum.example.com/*", zone_name = "continuum.example.com" }

[env.staging]
name = "continuum-api-staging"
route = { pattern = "api-staging.continuum.example.com/*", zone_name = "continuum.example.com" }

# KV名前空間
[[kv_namespaces]]
binding = "CACHE"
id = "your_kv_namespace_id"
preview_id = "your_preview_kv_namespace_id"

# 環境変数
[vars]
ENV = "production"
```

### 3. Workers用エントリーポイント (`server/src/worker.rs`)

**注意:** Leptos + AxumをWorkersで動作させるには、`workerd`互換のアダプターが必要な可能性があります。

**実装方針:**
- `leptos_axum`の代わりにWorkers用アダプターを使用
- または、Workers用のカスタムハンドラーを実装

**例（仮想実装）:**
```rust
use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    // Leptosアプリケーションの初期化
    // リクエストの処理
    // レスポンスの返却
}
```

### 4. ビルド設定

#### 4.1 Cargo.toml設定

```toml
# server/Cargo.toml
[package]
name = "server"
# ...

[lib]
crate-type = ["cdylib"]

[dependencies]
worker = "0.1"
# ...
```

#### 4.2 ビルドスクリプト

**注意:** LeptosのSSRをWorkersで動作させるには、特別なビルド設定が必要な可能性があります。

**検討事項:**
- `wasm-pack`を使用したWASMビルド
- Workers用のランタイム設定
- メモリ制限への対応

### 5. KV名前空間設定

**手順:**
1. Cloudflare Dashboard > Workers & Pages > KV
2. "Create a namespace" をクリック
3. 名前空間名を設定（例: `continuum-cache`）
4. Namespace IDを取得
5. `wrangler.toml`に設定

### 6. 環境変数設定

**Cloudflare Dashboard設定:**
1. Workers & Pages > continuum-api > Settings > Variables
2. 環境変数を追加:
   - `GITHUB_CLIENT_ID`
   - `GITHUB_CLIENT_SECRET`
   - `GITHUB_ORG_NAME`
   - `SESSION_SECRET`

**ローカル開発用 (.dev.vars):**
```bash
GITHUB_CLIENT_ID=your_client_id
GITHUB_CLIENT_SECRET=your_client_secret
GITHUB_ORG_NAME=your_org_name
SESSION_SECRET=your_secret
```

### 7. デプロイ手順

```bash
# ビルド
cargo leptos build --release

# Workers用ビルド（必要に応じて）
wasm-pack build --target web --out-dir target/server/release

# デプロイ
wrangler deploy

# ステージング環境にデプロイ
wrangler deploy --env staging
```

### 8. 制限事項と対応

**制限事項:**
- WorkersのCPU時間制限（10ms/リクエスト）
- メモリ制限（128MB）
- リクエストサイズ制限（100MB）

**対応:**
- 重い処理は非同期で実行
- キャッシュを積極的に使用
- レスポンスサイズの最適化

### 9. テスト

**テスト項目:**
- ローカルでのWorkers実行テスト
- デプロイ後の動作確認
- KV接続テスト
- 環境変数読み込みテスト

## 実装手順

1. Cloudflareアカウント作成・設定
2. `wrangler.toml` 作成
3. KV名前空間作成
4. 環境変数設定
5. Workers用エントリーポイント実装（必要に応じて）
6. ビルド設定調整
7. デプロイテスト

## 注意事項

**重要:** LeptosのSSRをCloudflare Workersで動作させるには、以下のいずれかが必要です：

1. **Workers互換アダプターの使用**
   - `leptos_axum`の代わりにWorkers用アダプターを探す
   - または、カスタムアダプターを実装

2. **アーキテクチャの見直し**
   - SSRをWorkersで行わず、Pagesで静的生成
   - APIのみWorkersで実装

3. **代替技術の検討**
   - Cloudflare Pages Functionsを使用
   - または、別のホスティングサービスを検討

## 関連ドキュメント

- 親計画: `../20250101_01_setup-plan.md`
- Cloudflare Pages設定: `../phase3-cloudflare-pages/README.md`
- CI/CD設定: `../phase3-cicd/README.md`
- キャッシュ戦略: `../phase2-caching-strategy/README.md`

