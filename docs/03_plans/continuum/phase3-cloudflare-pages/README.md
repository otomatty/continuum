# Phase 3-2: Cloudflare Pages設定

## 概要

LeptosフロントエンドをCloudflare Pagesにデプロイするための設定を行います。

## 目標

- Cloudflare Pages用のビルド設定
- 静的アセットの配信設定
- 環境変数の設定
- カスタムドメイン設定

## 実装内容

### 1. ファイル構成

```
.
├── wrangler.toml       # Cloudflare Pages設定（Workersと統合可能）
├── .dev.vars           # ローカル開発用環境変数
└── public/             # 静的アセット
```

### 2. Cloudflare Pages設定

#### 2.1 ビルド設定

**ビルドコマンド:**
```bash
cargo leptos build --release
```

**ビルド出力ディレクトリ:**
```
target/site
```

**ルートディレクトリ:**
```
target/site
```

#### 2.2 環境変数設定

**Cloudflare Dashboard設定:**
1. Pages > continuum-frontend > Settings > Environment variables
2. 環境変数を追加:
   - `LEPTOS_SITE_ROOT`
   - `LEPTOS_SITE_PKG_DIR`
   - `API_BASE_URL` (Workers APIのURL)

**ビルド時環境変数:**
```bash
LEPTOS_SITE_ROOT=target/site
LEPTOS_SITE_PKG_DIR=pkg
LEPTOS_OUTPUT_NAME=continuum
```

**ランタイム環境変数:**
```bash
API_BASE_URL=https://api.continuum.example.com
```

### 3. ビルド設定ファイル

#### 3.1 `_redirects` ファイル（SPA用）

```
# public/_redirects
/*    /index.html   200
```

#### 3.2 `_headers` ファイル（セキュリティヘッダー）

```
# public/_headers
/*
  X-Frame-Options: DENY
  X-Content-Type-Options: nosniff
  X-XSS-Protection: 1; mode=block
  Referrer-Policy: strict-origin-when-cross-origin
```

### 4. デプロイ手順

#### 4.1 GitHub連携による自動デプロイ

1. Cloudflare Dashboard > Pages > Create a project
2. "Connect to Git" を選択
3. GitHubリポジトリを選択
4. ビルド設定を入力:
   - Framework preset: None
   - Build command: `cargo leptos build --release`
   - Build output directory: `target/site`
5. 環境変数を設定
6. "Save and Deploy" をクリック

#### 4.2 手動デプロイ

```bash
# ビルド
cargo leptos build --release

# Wranglerを使用したデプロイ
wrangler pages deploy target/site --project-name=continuum-frontend
```

### 5. カスタムドメイン設定

**手順:**
1. Cloudflare Dashboard > Pages > continuum-frontend > Custom domains
2. "Set up a custom domain" をクリック
3. ドメイン名を入力（例: `continuum.example.com`）
4. DNS設定を確認（自動設定される場合あり）

### 6. プレビューデプロイ

**機能:**
- プルリクエストごとにプレビュー環境が自動生成
- ステージング環境として使用可能

**設定:**
- Cloudflare Dashboard > Pages > Settings > Builds & deployments
- "Preview deployments" を有効化

### 7. パフォーマンス最適化

#### 7.1 アセット最適化

- WASMファイルの圧縮
- CSS/JSの最適化
- 画像の最適化

#### 7.2 キャッシュ設定

**Cloudflare Dashboard設定:**
- Caching > Configuration
- Browser Cache TTL: 1年
- Edge Cache TTL: 1時間

### 8. エラーハンドリング

#### 8.1 404エラー

**設定:**
- `public/404.html` を作成
- または、`_redirects`でリダイレクト設定

#### 8.2 500エラー

**設定:**
- Cloudflare Dashboard > Pages > Settings > Error pages
- カスタムエラーページを設定

### 9. テスト

**テスト項目:**
- ビルド成功確認
- デプロイ後の動作確認
- ルーティングテスト
- API接続テスト
- 環境変数読み込みテスト

## 実装手順

1. Cloudflare Pagesプロジェクト作成
2. GitHubリポジトリ連携
3. ビルド設定
4. 環境変数設定
5. カスタムドメイン設定（オプション）
6. デプロイテスト
7. パフォーマンス最適化設定

## 注意事項

**WASM配信:**
- Cloudflare PagesはWASMファイルの配信をサポート
- MIMEタイプは自動設定される
- 必要に応じて`_headers`で明示的に設定

**SSR vs 静的生成:**
- 初期実装は静的生成を推奨
- SSRが必要な場合は、Pages Functionsを使用

## 関連ドキュメント

- 親計画: `../20250101_01_setup-plan.md`
- Cloudflare Workers設定: `../phase3-cloudflare-workers/README.md`
- CI/CD設定: `../phase3-cicd/README.md`

