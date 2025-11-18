# Phase 2-2: APIエンドポイント実装

## 概要

フロントエンドから使用するRESTful APIエンドポイントを実装します。

## 目標

- 組織情報、リポジトリ、ユーザー情報を取得するAPIエンドポイントの実装
- 認証・認可の実装
- エラーハンドリングとレスポンス形式の統一

## 実装内容

### 1. ディレクトリ構造

```
server/src/api/
├── mod.rs              # モジュール定義・ルーティング
├── handlers/
│   ├── mod.rs
│   ├── organization.rs # 組織情報取得
│   ├── repositories.rs # リポジトリ一覧
│   ├── users.rs        # ユーザー情報
│   └── dashboard.rs    # ダッシュボードデータ
└── responses.rs        # 共通レスポンス型
```

### 2. APIエンドポイント一覧

#### 2.1 組織情報取得

**エンドポイント:** `GET /api/organization`

**レスポンス:**
```json
{
  "data": {
    "login": "organization-name",
    "name": "Organization Name",
    "description": "Description",
    "avatar_url": "https://...",
    "member_count": 50,
    "repository_count": 30
  }
}
```

**実装:** `handlers/organization.rs`

#### 2.2 リポジトリ一覧取得

**エンドポイント:** `GET /api/repositories`

**クエリパラメータ:**
- `page`: ページ番号（デフォルト: 1）
- `per_page`: 1ページあたりの件数（デフォルト: 30）
- `sort`: ソート順（stars, updated, created）
- `order`: 昇順/降順（asc, desc）

**レスポンス:**
```json
{
  "data": [
    {
      "id": "...",
      "name": "repo-name",
      "description": "...",
      "stargazer_count": 100,
      "updated_at": "2024-01-01T00:00:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "per_page": 30,
    "total": 30,
    "total_pages": 1
  }
}
```

**実装:** `handlers/repositories.rs`

#### 2.3 ユーザー情報取得

**エンドポイント:** `GET /api/users/:username`

**パスパラメータ:**
- `username`: GitHubユーザー名

**レスポンス:**
```json
{
  "data": {
    "user": {
      "id": "...",
      "login": "username",
      "name": "User Name",
      "avatar_url": "https://..."
    },
    "contribution_summary": {
      "total_commits": 100,
      "total_pull_requests": 50,
      "total_reviews": 200
    },
    "repositories": [...]
  }
}
```

**実装:** `handlers/users.rs`

#### 2.4 ダッシュボードデータ取得

**エンドポイント:** `GET /api/dashboard`

**レスポンス:**
```json
{
  "data": {
    "organization": {...},
    "summary": {
      "total_contributors": 50,
      "total_repositories": 30,
      "external_pr_count": 10
    },
    "activity_timeline": [...],
    "top_repositories": [...],
    "top_contributors": [...]
  }
}
```

**実装:** `handlers/dashboard.rs`

### 3. 共通レスポンス型 (`responses.rs`)

```rust
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub error: String,
    pub message: String,
    pub code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: Pagination,
}

#[derive(Debug, Serialize)]
pub struct Pagination {
    pub page: u32,
    pub per_page: u32,
    pub total: u32,
    pub total_pages: u32,
}
```

### 4. エラーハンドリング

**エラー種別:**
- `NotFound` (404)
- `Unauthorized` (401)
- `Forbidden` (403)
- `BadRequest` (400)
- `InternalServerError` (500)

**実装:**
```rust
pub async fn handle_error(error: ApiError) -> impl IntoResponse {
    let status = match error.code.as_deref() {
        Some("NOT_FOUND") => StatusCode::NOT_FOUND,
        Some("UNAUTHORIZED") => StatusCode::UNAUTHORIZED,
        // ...
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    };
    
    (status, Json(error))
}
```

### 5. 認証・認可

**実装:**
- 認証ミドルウェアを使用（Phase 1-2で実装）
- 一部エンドポイントは認証不要（`/api/organization`等）
- ユーザー固有データは認証必須

### 6. キャッシュ統合

**実装:**
- Cloudflare KV互換のキャッシュレイヤーを使用
- キャッシュキー設計（Phase 2-3で詳細化）
- キャッシュ無効化戦略

### 7. ルーティング設定 (`mod.rs`)

```rust
use axum::Router;

pub fn create_api_router() -> Router {
    Router::new()
        .route("/organization", get(handlers::organization::get_organization))
        .route("/repositories", get(handlers::repositories::list_repositories))
        .route("/users/:username", get(handlers::users::get_user))
        .route("/dashboard", get(handlers::dashboard::get_dashboard))
}
```

### 8. テスト

**テスト項目:**
- 各エンドポイントの統合テスト
- 認証・認可テスト
- エラーハンドリングテスト
- ページネーションテスト

## 実装手順

1. `server/src/api/` ディレクトリ作成
2. `responses.rs` で共通レスポンス型定義
3. 各ハンドラーを順次実装
4. ルーティング設定
5. エラーハンドリング実装
6. 認証ミドルウェア統合
7. テスト作成

## 関連ドキュメント

- 親計画: `../20250101_01_setup-plan.md`
- データモデル: `../phase2-data-models/README.md`
- キャッシュ戦略: `../phase2-caching-strategy/README.md`
- GitHub OAuth: `../phase1-github-oauth/README.md`

