# Task 5: GitHub API 実装

## 1. 目的と背景

### なぜこのタスクが必要か
Continuumはモックデータではなく、実際のGitHub Organization のデータを表示する必要があります。GitHub GraphQL API (v4) を使用して、リポジトリ、コントリビューター、コミット、プルリクエストなどの情報を取得します。

### 完成時のユーザー体験
- ダッシュボードに実際の組織統計が表示される
- ポートフォリオページに実際のコントリビューション履歴が表示される
- データは適切にキャッシュされ、高速に表示される

---

## 2. 前提条件

### 依存するタスク
- ✅ Task 1-4: Phase 1 完了

### 必要な環境設定
1. GitHub Personal Access Token（または OAuth App）
2. 対象となるGitHub Organization名
3. `.env` ファイルへの環境変数設定

### 必要な知識
- GitHub GraphQL API の基本
- Rust での HTTP リクエスト（reqwest）
- 非同期処理（async/await）

---

## 3. 作成/更新ファイル一覧

### 新規作成ファイル
| ファイル | 内容 |
|---------|------|
| `app/src/github/queries.rs` | GraphQL クエリ定義（拡充） |
| `app/src/github/client.rs` | API クライアント（拡充） |
| `app/src/github/types.rs` | レスポンス型定義（拡充） |
| `app/src/concepts/github_data/state.rs` | GitHub データの状態管理 |
| `app/src/concepts/github_data/actions.rs` | データ取得ロジック |
| `app/src/concepts/github_data/mod.rs` | モジュール定義 |
| `app/src/concepts/github_data/github_data.spec.md` | 仕様書 |
| `app/src/concepts/github_data/tests.rs` | テスト |

### 更新ファイル
| ファイル | 変更内容 |
|---------|---------|
| `app/src/concepts/mod.rs` | `github_data` モジュール追加 |
| `server/src/config.rs` | GitHub 設定追加（必要に応じて） |

---

## 4. 実装手順

### Step 1: 環境変数の設定

`.env` ファイルに以下を追加（存在しない場合は作成）：

```bash
# GitHub Organization name
GITHUB_ORG=your-organization-name

# GitHub API Token (Personal Access Token with read:org, repo scopes)
# OAuth認証済みユーザーの場合はセッションから取得
```

### Step 2: GraphQL クエリの定義

`app/src/github/queries.rs` を更新：

```rust
/// Organization の統計情報を取得するクエリ
pub const ORGANIZATION_STATS_QUERY: &str = r#"
query OrganizationStats($org: String!) {
  organization(login: $org) {
    repositories(first: 100, privacy: PUBLIC) {
      totalCount
      nodes {
        name
        stargazerCount
        forkCount
        updatedAt
        primaryLanguage {
          name
          color
        }
      }
    }
    membersWithRole {
      totalCount
    }
  }
}
"#;

/// ユーザーのコントリビューション情報を取得するクエリ
pub const USER_CONTRIBUTIONS_QUERY: &str = r#"
query UserContributions($username: String!, $org: String!, $from: DateTime!, $to: DateTime!) {
  user(login: $username) {
    contributionsCollection(organizationID: $org, from: $from, to: $to) {
      totalCommitContributions
      totalPullRequestContributions
      totalPullRequestReviewContributions
      totalIssueContributions
      contributionCalendar {
        totalContributions
        weeks {
          contributionDays {
            contributionCount
            date
          }
        }
      }
    }
  }
}
"#;

/// リポジトリ一覧を取得するクエリ
pub const REPOSITORIES_QUERY: &str = r#"
query Repositories($org: String!, $first: Int!, $after: String) {
  organization(login: $org) {
    repositories(first: $first, after: $after, privacy: PUBLIC, orderBy: {field: UPDATED_AT, direction: DESC}) {
      pageInfo {
        hasNextPage
        endCursor
      }
      nodes {
        name
        description
        url
        stargazerCount
        forkCount
        updatedAt
        primaryLanguage {
          name
          color
        }
        contributors: mentionableUsers(first: 10) {
          totalCount
          nodes {
            login
            avatarUrl
          }
        }
      }
    }
  }
}
"#;
```

### Step 3: 型定義の追加

`app/src/github/types.rs` を更新：

```rust
use serde::{Deserialize, Serialize};

/// GitHub GraphQL API のレスポンス構造
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLResponse<T> {
    pub data: Option<T>,
    pub errors: Option<Vec<GraphQLError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLError {
    pub message: String,
}

/// Organization 統計のレスポンス
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationStatsData {
    pub organization: Option<Organization>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    pub repositories: RepositoryConnection,
    #[serde(rename = "membersWithRole")]
    pub members_with_role: MemberConnection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryConnection {
    #[serde(rename = "totalCount")]
    pub total_count: i32,
    pub nodes: Vec<Repository>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub name: String,
    pub description: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "stargazerCount")]
    pub stargazer_count: i32,
    #[serde(rename = "forkCount")]
    pub fork_count: i32,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "primaryLanguage")]
    pub primary_language: Option<Language>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Language {
    pub name: String,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberConnection {
    #[serde(rename = "totalCount")]
    pub total_count: i32,
}
```

### Step 4: API クライアントの実装

`app/src/github/client.rs` を更新：

```rust
use super::queries::*;
use super::types::*;
use serde_json::json;

pub struct GitHubClient {
    http_client: reqwest::Client,
    token: String,
}

impl GitHubClient {
    pub fn new(token: String) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            token,
        }
    }

    /// GraphQL クエリを実行
    async fn execute_query<T: serde::de::DeserializeOwned>(
        &self,
        query: &str,
        variables: serde_json::Value,
    ) -> Result<T, GitHubApiError> {
        let response = self
            .http_client
            .post("https://api.github.com/graphql")
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "continuum")
            .json(&json!({
                "query": query,
                "variables": variables
            }))
            .send()
            .await
            .map_err(|e| GitHubApiError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(GitHubApiError::HttpError(response.status().as_u16()));
        }

        let graphql_response: GraphQLResponse<T> = response
            .json()
            .await
            .map_err(|e| GitHubApiError::ParseError(e.to_string()))?;

        if let Some(errors) = graphql_response.errors {
            return Err(GitHubApiError::GraphQLError(
                errors.into_iter().map(|e| e.message).collect(),
            ));
        }

        graphql_response
            .data
            .ok_or(GitHubApiError::NoData)
    }

    /// Organization の統計情報を取得
    pub async fn get_organization_stats(
        &self,
        org: &str,
    ) -> Result<OrganizationStatsData, GitHubApiError> {
        self.execute_query(
            ORGANIZATION_STATS_QUERY,
            json!({ "org": org }),
        )
        .await
    }

    /// リポジトリ一覧を取得
    pub async fn get_repositories(
        &self,
        org: &str,
        first: i32,
        after: Option<&str>,
    ) -> Result<RepositoriesData, GitHubApiError> {
        self.execute_query(
            REPOSITORIES_QUERY,
            json!({
                "org": org,
                "first": first,
                "after": after
            }),
        )
        .await
    }
}

#[derive(Debug, Clone)]
pub enum GitHubApiError {
    NetworkError(String),
    HttpError(u16),
    ParseError(String),
    GraphQLError(Vec<String>),
    NoData,
}
```

### Step 5: GitHub Data Concept の作成

`app/src/concepts/github_data/state.rs`:

```rust
use serde::{Deserialize, Serialize};

/// GitHub から取得したデータの状態
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GitHubDataState {
    pub organization_stats: Option<OrganizationStats>,
    pub repositories: Vec<RepositoryInfo>,
    pub loading: bool,
    pub error: Option<String>,
}

/// 組織の統計情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationStats {
    pub total_contributors: i32,
    pub total_repositories: i32,
    pub external_prs_count: i32,
}

/// リポジトリ情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryInfo {
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub stars: i32,
    pub forks: i32,
    pub language: Option<String>,
    pub updated_at: String,
}
```

### Step 6: Server Function の実装

Leptos Server Function を使用してサーバーサイドで API を呼び出す：

```rust
// app/src/concepts/github_data/actions.rs

#[cfg(feature = "ssr")]
use crate::github::client::GitHubClient;

use super::state::*;
use leptos::prelude::*;

#[server(GetOrganizationStats, "/api/github/stats")]
pub async fn get_organization_stats() -> Result<OrganizationStats, ServerFnError> {
    let org = std::env::var("GITHUB_ORG")
        .map_err(|_| ServerFnError::new("GITHUB_ORG not set"))?;
    
    // OAuth認証済みユーザーのトークンを使用
    // または環境変数からトークンを取得
    let token = std::env::var("GITHUB_TOKEN")
        .map_err(|_| ServerFnError::new("GITHUB_TOKEN not set"))?;

    let client = GitHubClient::new(token);
    
    let data = client
        .get_organization_stats(&org)
        .await
        .map_err(|e| ServerFnError::new(format!("GitHub API error: {:?}", e)))?;

    let org_data = data.organization
        .ok_or_else(|| ServerFnError::new("Organization not found"))?;

    Ok(OrganizationStats {
        total_contributors: org_data.members_with_role.total_count,
        total_repositories: org_data.repositories.total_count,
        external_prs_count: 0, // TODO: 外部PRの集計ロジックを追加
    })
}
```

### Step 7: レート制限への対応

GitHub API にはレート制限があります。以下の対策を実装：

1. **レート制限ヘッダーの確認**
   - `X-RateLimit-Remaining`: 残りリクエスト数
   - `X-RateLimit-Reset`: リセット時刻

2. **キャッシュの実装**（Task 5.3）
   - 頻繁に変わらないデータはキャッシュ
   - TTL: 5分〜1時間（データの種類による）

---

## 5. テスト項目

### 単体テスト

```rust
// app/src/concepts/github_data/tests.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organization_stats_default() {
        let state = GitHubDataState::default();
        assert!(state.organization_stats.is_none());
        assert!(state.repositories.is_empty());
        assert!(!state.loading);
        assert!(state.error.is_none());
    }

    #[test]
    fn test_repository_info_creation() {
        let repo = RepositoryInfo {
            name: "test-repo".to_string(),
            description: Some("A test repository".to_string()),
            url: "https://github.com/org/test-repo".to_string(),
            stars: 100,
            forks: 20,
            language: Some("Rust".to_string()),
            updated_at: "2025-01-01T00:00:00Z".to_string(),
        };
        
        assert_eq!(repo.name, "test-repo");
        assert_eq!(repo.stars, 100);
    }
}
```

### 動作確認手順

1. 環境変数を設定
2. 開発サーバーを起動
3. ダッシュボードにアクセス
4. 統計情報がリアルデータに更新されていることを確認

```bash
# 環境変数を設定
export GITHUB_ORG="your-org"
export GITHUB_TOKEN="ghp_xxxxx"

# 開発サーバー起動
bun run dev
```

---

## 6. 完了条件チェックリスト

- [ ] GraphQL クエリが定義されている
- [ ] 型定義が完了している
- [ ] GitHubClient が実装されている
- [ ] Server Function が実装されている
- [ ] エラーハンドリングが適切に行われている
- [ ] レート制限への対応が考慮されている
- [ ] テストが通る
- [ ] 実際のデータが取得できる

---

## 7. 参照ドキュメント

- GitHub GraphQL API: https://docs.github.com/en/graphql
- GitHub GraphQL Explorer: https://docs.github.com/en/graphql/overview/explorer
- 既存の GitHub 関連コード: `app/src/github/`
- PRD: `PRD.md` - セクション 7.1

---

## 8. 注意点

- **トークンの管理**: GitHub Token は絶対にクライアントサイドに露出しない
- **レート制限**: GitHub API は 1時間あたり 5,000 リクエストの制限がある
- **エラーハンドリング**: API エラー時はユーザーに適切なメッセージを表示
- **ページネーション**: 大量のデータを扱う場合はページネーションを実装

