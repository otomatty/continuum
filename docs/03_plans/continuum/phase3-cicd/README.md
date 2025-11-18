# Phase 3-3: CI/CD設定

## 概要

GitHub Actionsを使用した自動テスト・デプロイパイプラインを構築します。

## 目標

- 自動テスト実行
- 自動ビルド・デプロイ
- プルリクエストごとのプレビューデプロイ
- 本番環境への自動デプロイ

## 実装内容

### 1. ファイル構成

```
.github/
└── workflows/
    ├── ci.yml          # 継続的インテグレーション
    ├── deploy-workers.yml  # Workersデプロイ
    └── deploy-pages.yml    # Pagesデプロイ
```

### 2. CIパイプライン (`ci.yml`)

#### 2.1 ワークフロー定義

```yaml
name: CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main, develop]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: nightly
          target: wasm32-unknown-unknown
          override: true
      
      - name: Install cargo-leptos
        run: cargo install cargo-leptos --locked
      
      - name: Run tests
        run: cargo test --all
      
      - name: Run end-to-end tests
        run: cargo leptos end-to-end --release
        env:
          LEPTOS_SITE_ADDR: 127.0.0.1:3000
```

#### 2.2 テスト項目

- ユニットテスト
- 統合テスト
- end-to-endテスト（Playwright）
- リントチェック（clippy）
- フォーマットチェック（rustfmt）

### 3. Workersデプロイパイプライン (`deploy-workers.yml`)

```yaml
name: Deploy Workers

on:
  push:
    branches: [main]
    paths:
      - 'server/**'
      - 'app/**'
      - 'Cargo.toml'
      - 'wrangler.toml'
  workflow_dispatch:

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: nightly
          target: wasm32-unknown-unknown
          override: true
      
      - name: Install cargo-leptos
        run: cargo install cargo-leptos --locked
      
      - name: Install wrangler
        run: npm install -g wrangler
      
      - name: Build
        run: cargo leptos build --release
      
      - name: Deploy to Cloudflare Workers
        run: wrangler deploy
        env:
          CLOUDFLARE_API_TOKEN: ${{ secrets.CLOUDFLARE_API_TOKEN }}
          CLOUDFLARE_ACCOUNT_ID: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
```

### 4. Pagesデプロイパイプライン (`deploy-pages.yml`)

```yaml
name: Deploy Pages

on:
  push:
    branches: [main]
    paths:
      - 'app/**'
      - 'frontend/**'
      - 'style/**'
      - 'public/**'
      - 'Cargo.toml'
  workflow_dispatch:

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: nightly
          target: wasm32-unknown-unknown
          override: true
      
      - name: Install cargo-leptos
        run: cargo install cargo-leptos --locked
      
      - name: Install wrangler
        run: npm install -g wrangler
      
      - name: Build
        run: cargo leptos build --release
        env:
          LEPTOS_SITE_ROOT: target/site
          LEPTOS_SITE_PKG_DIR: pkg
          LEPTOS_OUTPUT_NAME: continuum
      
      - name: Deploy to Cloudflare Pages
        run: wrangler pages deploy target/site --project-name=continuum-frontend
        env:
          CLOUDFLARE_API_TOKEN: ${{ secrets.CLOUDFLARE_API_TOKEN }}
          CLOUDFLARE_ACCOUNT_ID: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
```

### 5. プレビューデプロイ

**機能:**
- プルリクエストごとにプレビュー環境をデプロイ
- レビュー前に動作確認可能

**実装:**
```yaml
name: Preview Deploy

on:
  pull_request:
    branches: [main]

jobs:
  preview:
    runs-on: ubuntu-latest
    steps:
      # ... ビルドステップ ...
      
      - name: Deploy Preview
        run: wrangler pages deploy target/site --project-name=continuum-frontend
        env:
          CLOUDFLARE_API_TOKEN: ${{ secrets.CLOUDFLARE_API_TOKEN }}
```

### 6. シークレット設定

**GitHub Secrets設定:**
1. Repository > Settings > Secrets and variables > Actions
2. 以下のシークレットを追加:
   - `CLOUDFLARE_API_TOKEN`
   - `CLOUDFLARE_ACCOUNT_ID`
   - `GITHUB_CLIENT_ID` (必要に応じて)
   - `GITHUB_CLIENT_SECRET` (必要に応じて)

### 7. デプロイ戦略

#### 7.1 ブランチ戦略

- `main`: 本番環境に自動デプロイ
- `develop`: ステージング環境に自動デプロイ
- `feature/*`: プレビュー環境にデプロイ

#### 7.2 デプロイ承認

**本番環境デプロイ:**
- 手動承認を要求（オプション）
- または、自動デプロイ

**実装:**
```yaml
jobs:
  deploy-production:
    # ...
    environment:
      name: production
      url: https://continuum.example.com
```

### 8. 通知設定

**Slack通知（オプション）:**
```yaml
- name: Notify Slack
  if: failure()
  uses: slackapi/slack-github-action@v1
  with:
    webhook-url: ${{ secrets.SLACK_WEBHOOK_URL }}
    payload: |
      {
        "text": "Deployment failed"
      }
```

### 9. キャッシュ最適化

**Rustキャッシュ:**
```yaml
- name: Cache Rust dependencies
  uses: actions/cache@v3
  with:
    path: |
      ~/.cargo/bin/
      ~/.cargo/registry/index/
      ~/.cargo/registry/cache/
      ~/.cargo/git/db/
      target/
    key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
```

## 実装手順

1. `.github/workflows/` ディレクトリ作成
2. `ci.yml` でCIパイプライン実装
3. `deploy-workers.yml` でWorkersデプロイ実装
4. `deploy-pages.yml` でPagesデプロイ実装
5. GitHub Secrets設定
6. デプロイテスト
7. 通知設定（オプション）

## 関連ドキュメント

- 親計画: `../20250101_01_setup-plan.md`
- Cloudflare Workers設定: `../phase3-cloudflare-workers/README.md`
- Cloudflare Pages設定: `../phase3-cloudflare-pages/README.md`

