# Phase 2-1: データモデル定義

## 概要

GitHub APIから取得したデータを扱うためのドメインモデルを定義します。

## 目標

- GitHub APIレスポンスの型定義
- ドメインモデル（User, Repository, Contribution等）の定義
- シリアライゼーション/デシリアライゼーション対応

## 実装内容

### 1. ディレクトリ構造

```
app/src/models/
├── mod.rs              # モジュール定義
├── organization.rs     # 組織モデル
├── repository.rs       # リポジトリモデル
├── user.rs             # ユーザーモデル
├── contribution.rs     # コントリビューションモデル
└── common.rs           # 共通型定義
```

### 2. データモデル定義

#### 2.1 Organization (`organization.rs`)

```rust
pub struct Organization {
    pub login: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub avatar_url: String,
    pub website_url: Option<String>,
    pub member_count: u32,
    pub repository_count: u32,
}
```

#### 2.2 Repository (`repository.rs`)

```rust
pub struct Repository {
    pub id: String,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub url: String,
    pub homepage_url: Option<String>,
    pub language: Option<String>,
    pub languages: Vec<Language>,
    pub stargazer_count: u32,
    pub fork_count: u32,
    pub is_archived: bool,
    pub is_disabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub pushed_at: Option<DateTime<Utc>>,
    pub topics: Vec<String>,
    pub default_branch: String,
}

pub struct Language {
    pub name: String,
    pub size: u64,
}
```

#### 2.3 User (`user.rs`)

```rust
pub struct User {
    pub id: String,
    pub login: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: String,
    pub bio: Option<String>,
    pub company: Option<String>,
    pub blog: Option<String>,
    pub location: Option<String>,
    pub created_at: DateTime<Utc>,
    pub organization_member_since: Option<DateTime<Utc>>,
}

pub struct UserProfile {
    pub user: User,
    pub contribution_summary: ContributionSummary,
    pub repositories: Vec<RepositoryContribution>,
}
```

#### 2.4 Contribution (`contribution.rs`)

```rust
pub struct Contribution {
    pub id: String,
    pub user: User,
    pub repository: Repository,
    pub commits: Vec<Commit>,
    pub pull_requests: Vec<PullRequest>,
    pub reviews: Vec<Review>,
    pub period: ContributionPeriod,
}

pub struct ContributionSummary {
    pub total_commits: u32,
    pub total_pull_requests: u32,
    pub total_reviews: u32,
    pub total_repositories: u32,
    pub contribution_graph: ContributionGraph,
}

pub struct ContributionGraph {
    pub data: Vec<ContributionDay>,
}

pub struct ContributionDay {
    pub date: Date<Utc>,
    pub commit_count: u32,
    pub pr_count: u32,
    pub review_count: u32,
}

pub struct Commit {
    pub id: String,
    pub message: String,
    pub sha: String,
    pub url: String,
    pub authored_date: DateTime<Utc>,
    pub committed_date: DateTime<Utc>,
}

pub struct PullRequest {
    pub id: String,
    pub number: u32,
    pub title: String,
    pub body: Option<String>,
    pub state: PullRequestState,
    pub url: String,
    pub created_at: DateTime<Utc>,
    pub merged_at: Option<DateTime<Utc>>,
    pub closed_at: Option<DateTime<Utc>>,
}

pub struct Review {
    pub id: String,
    pub state: ReviewState,
    pub body: Option<String>,
    pub submitted_at: DateTime<Utc>,
}

pub enum PullRequestState {
    Open,
    Closed,
    Merged,
}

pub enum ReviewState {
    Approved,
    ChangesRequested,
    Commented,
    Dismissed,
}

pub enum ContributionPeriod {
    All,
    Year { year: u32 },
    Month { year: u32, month: u32 },
    Week { start: Date<Utc>, end: Date<Utc> },
}
```

#### 2.5 Common (`common.rs`)

```rust
use chrono::{DateTime, Date, Utc};

pub type Timestamp = DateTime<Utc>;
pub type DateOnly = Date<Utc>;
```

### 3. シリアライゼーション

**実装方針:**
- `serde` を使用したJSONシリアライゼーション
- GitHub APIレスポンスからの変換関数
- フロントエンドへの送信用シリアライゼーション

**例:**
```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    // ...
}

impl From<github::RepositoryResponse> for Repository {
    fn from(response: github::RepositoryResponse) -> Self {
        // GitHub APIレスポンスからドメインモデルへの変換
    }
}
```

### 4. 依存関係

```toml
# app/Cargo.toml に追加
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
```

### 5. バリデーション

**実装項目:**
- 必須フィールドのチェック
- 日付の妥当性チェック
- URL形式のチェック

### 6. テスト

**テスト項目:**
- GitHub APIレスポンスからの変換テスト
- シリアライゼーション/デシリアライゼーションテスト
- バリデーションテスト

## 実装手順

1. `app/src/models/` ディレクトリ作成
2. `mod.rs` でモジュール定義
3. `common.rs` で共通型定義
4. 各モデルファイルを順次実装
5. GitHub APIレスポンスからの変換実装
6. テスト作成

## 関連ドキュメント

- 親計画: `../20250101_01_setup-plan.md`
- GitHub API クライアント: `../phase1-github-api-client/README.md`
- APIエンドポイント: `../phase2-api-endpoints/README.md`

