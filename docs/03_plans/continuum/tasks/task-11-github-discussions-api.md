# Task 11: GitHub Discussions API 連携

## 1. 目的と背景

### なぜこのタスクが必要か
Task 10 で定義した Discussion Concept に実際のデータを流し込むため、GitHub GraphQL API を使用して Discussions データを取得する機能を実装します。

### 完成時のユーザー体験
- 知見共有ページにリアルな GitHub Discussions が表示される
- カテゴリ別にフィルタリングできる
- 詳細ページで Markdown がレンダリングされる

---

## 2. 前提条件

### 依存するタスク
- ✅ Task 5: GitHub API 実装（基盤）
- ✅ Task 10: Discussion Concepts 実装

### 必要な環境設定
- GitHub Organization に Discussions が有効なリポジトリが存在すること
- `GITHUB_ORG` 環境変数が設定されていること

---

## 3. 作成/更新ファイル一覧

### 更新ファイル
| ファイル | 変更内容 |
|---------|---------|
| `app/src/github/queries.rs` | Discussions 関連クエリ追加 |
| `app/src/github/types.rs` | Discussions レスポンス型追加 |
| `app/src/github/client.rs` | Discussions 取得メソッド追加 |

### 新規作成ファイル
| ファイル | 内容 |
|---------|------|
| `app/src/concepts/discussion/server.rs` | Server Function 定義 |

---

## 4. 実装手順

### Step 1: GraphQL クエリの追加

`app/src/github/queries.rs` に追加：

```rust
/// Discussion カテゴリ一覧を取得するクエリ
pub const DISCUSSION_CATEGORIES_QUERY: &str = r#"
query DiscussionCategories($owner: String!, $name: String!) {
  repository(owner: $owner, name: $name) {
    discussionCategories(first: 25) {
      nodes {
        id
        name
        description
        emoji
      }
    }
  }
}
"#;

/// Discussion 一覧を取得するクエリ
pub const DISCUSSIONS_QUERY: &str = r#"
query Discussions(
  $owner: String!, 
  $name: String!, 
  $first: Int!, 
  $after: String,
  $categoryId: ID
) {
  repository(owner: $owner, name: $name) {
    discussions(
      first: $first, 
      after: $after, 
      categoryId: $categoryId,
      orderBy: {field: CREATED_AT, direction: DESC}
    ) {
      totalCount
      pageInfo {
        hasNextPage
        hasPreviousPage
        endCursor
        startCursor
      }
      nodes {
        id
        title
        body
        bodyText
        createdAt
        updatedAt
        url
        author {
          login
          avatarUrl
        }
        category {
          id
          name
          description
          emoji
        }
        comments {
          totalCount
        }
        reactions {
          totalCount
        }
        labels(first: 10) {
          nodes {
            name
          }
        }
      }
    }
  }
}
"#;

/// 単一の Discussion を取得するクエリ
pub const DISCUSSION_DETAIL_QUERY: &str = r#"
query DiscussionDetail($id: ID!) {
  node(id: $id) {
    ... on Discussion {
      id
      title
      body
      bodyHTML
      createdAt
      updatedAt
      url
      author {
        login
        avatarUrl
      }
      category {
        id
        name
        description
        emoji
      }
      comments(first: 50) {
        totalCount
        nodes {
          id
          body
          bodyHTML
          createdAt
          author {
            login
            avatarUrl
          }
          reactions {
            totalCount
          }
        }
      }
      reactions {
        totalCount
      }
      labels(first: 10) {
        nodes {
          name
        }
      }
    }
  }
}
"#;
```

### Step 2: 型定義の追加

`app/src/github/types.rs` に追加：

```rust
/// Discussion カテゴリのレスポンス
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscussionCategoriesData {
    pub repository: Option<DiscussionCategoriesRepository>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscussionCategoriesRepository {
    #[serde(rename = "discussionCategories")]
    pub discussion_categories: DiscussionCategoryConnection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscussionCategoryConnection {
    pub nodes: Vec<GitHubDiscussionCategory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubDiscussionCategory {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub emoji: Option<String>,
}

/// Discussions 一覧のレスポンス
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscussionsData {
    pub repository: Option<DiscussionsRepository>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscussionsRepository {
    pub discussions: DiscussionConnection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscussionConnection {
    #[serde(rename = "totalCount")]
    pub total_count: i32,
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
    pub nodes: Vec<GitHubDiscussion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageInfo {
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    #[serde(rename = "hasPreviousPage")]
    pub has_previous_page: bool,
    #[serde(rename = "endCursor")]
    pub end_cursor: Option<String>,
    #[serde(rename = "startCursor")]
    pub start_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubDiscussion {
    pub id: String,
    pub title: String,
    pub body: String,
    #[serde(rename = "bodyText")]
    pub body_text: Option<String>,
    #[serde(rename = "bodyHTML")]
    pub body_html: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub url: String,
    pub author: Option<GitHubAuthor>,
    pub category: GitHubDiscussionCategory,
    pub comments: CommentCount,
    pub reactions: ReactionCount,
    pub labels: LabelConnection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubAuthor {
    pub login: String,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentCount {
    #[serde(rename = "totalCount")]
    pub total_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionCount {
    #[serde(rename = "totalCount")]
    pub total_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelConnection {
    pub nodes: Vec<Label>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub name: String,
}
```

### Step 3: API クライアントにメソッド追加

`app/src/github/client.rs` に追加：

```rust
impl GitHubClient {
    /// Discussion カテゴリを取得
    pub async fn get_discussion_categories(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<DiscussionCategoriesData, GitHubApiError> {
        self.execute_query(
            DISCUSSION_CATEGORIES_QUERY,
            json!({
                "owner": owner,
                "name": repo
            }),
        )
        .await
    }

    /// Discussion 一覧を取得
    pub async fn get_discussions(
        &self,
        owner: &str,
        repo: &str,
        first: i32,
        after: Option<&str>,
        category_id: Option<&str>,
    ) -> Result<DiscussionsData, GitHubApiError> {
        self.execute_query(
            DISCUSSIONS_QUERY,
            json!({
                "owner": owner,
                "name": repo,
                "first": first,
                "after": after,
                "categoryId": category_id
            }),
        )
        .await
    }

    /// Discussion 詳細を取得
    pub async fn get_discussion_detail(
        &self,
        discussion_id: &str,
    ) -> Result<DiscussionDetailData, GitHubApiError> {
        self.execute_query(
            DISCUSSION_DETAIL_QUERY,
            json!({ "id": discussion_id }),
        )
        .await
    }
}
```

### Step 4: Server Function の実装

`app/src/concepts/discussion/server.rs`:

```rust
#[cfg(feature = "ssr")]
use crate::github::client::GitHubClient;

use super::state::*;
use leptos::prelude::*;

/// Discussion カテゴリ一覧を取得
#[server(GetDiscussionCategories, "/api/discussions/categories")]
pub async fn get_discussion_categories(
    repo_name: String,
) -> Result<Vec<crate::concepts::category::Category>, ServerFnError> {
    let org = std::env::var("GITHUB_ORG")
        .map_err(|_| ServerFnError::new("GITHUB_ORG not set"))?;
    
    let token = std::env::var("GITHUB_TOKEN")
        .map_err(|_| ServerFnError::new("GITHUB_TOKEN not set"))?;

    let client = GitHubClient::new(token);
    
    let data = client
        .get_discussion_categories(&org, &repo_name)
        .await
        .map_err(|e| ServerFnError::new(format!("GitHub API error: {:?}", e)))?;

    let repo = data.repository
        .ok_or_else(|| ServerFnError::new("Repository not found"))?;

    let categories = repo.discussion_categories.nodes
        .into_iter()
        .map(|c| crate::concepts::category::Category {
            id: c.id,
            name: c.name,
            description: c.description,
            emoji: c.emoji,
            discussions_count: 0, // 別途取得が必要
        })
        .collect();

    Ok(categories)
}

/// Discussion 一覧を取得
#[server(GetDiscussions, "/api/discussions")]
pub async fn get_discussions(
    repo_name: String,
    first: i32,
    after: Option<String>,
    category_id: Option<String>,
) -> Result<(Vec<Discussion>, PaginationInfo), ServerFnError> {
    let org = std::env::var("GITHUB_ORG")
        .map_err(|_| ServerFnError::new("GITHUB_ORG not set"))?;
    
    let token = std::env::var("GITHUB_TOKEN")
        .map_err(|_| ServerFnError::new("GITHUB_TOKEN not set"))?;

    let client = GitHubClient::new(token);
    
    let data = client
        .get_discussions(
            &org,
            &repo_name,
            first,
            after.as_deref(),
            category_id.as_deref(),
        )
        .await
        .map_err(|e| ServerFnError::new(format!("GitHub API error: {:?}", e)))?;

    let repo = data.repository
        .ok_or_else(|| ServerFnError::new("Repository not found"))?;

    let discussions = repo.discussions.nodes
        .into_iter()
        .map(|d| {
            let body_preview = d.body_text
                .as_ref()
                .map(|t| {
                    if t.len() > 200 {
                        format!("{}...", &t[..200])
                    } else {
                        t.clone()
                    }
                })
                .unwrap_or_default();

            Discussion {
                id: d.id,
                title: d.title,
                body: d.body,
                body_preview,
                author: DiscussionAuthor {
                    username: d.author.as_ref().map(|a| a.login.clone()).unwrap_or_default(),
                    display_name: None,
                    avatar_url: d.author.as_ref().map(|a| a.avatar_url.clone()).unwrap_or_default(),
                },
                category: DiscussionCategory {
                    id: d.category.id,
                    name: d.category.name,
                    description: d.category.description,
                    emoji: d.category.emoji,
                },
                created_at: d.created_at,
                updated_at: d.updated_at,
                comments_count: d.comments.total_count,
                reactions_count: d.reactions.total_count,
                url: d.url,
                labels: d.labels.nodes.into_iter().map(|l| l.name).collect(),
            }
        })
        .collect();

    let pagination = PaginationInfo {
        has_next_page: repo.discussions.page_info.has_next_page,
        has_previous_page: repo.discussions.page_info.has_previous_page,
        end_cursor: repo.discussions.page_info.end_cursor,
        start_cursor: repo.discussions.page_info.start_cursor,
        total_count: repo.discussions.total_count,
    };

    Ok((discussions, pagination))
}

/// Discussion 詳細を取得
#[server(GetDiscussionDetail, "/api/discussions/detail")]
pub async fn get_discussion_detail(
    discussion_id: String,
) -> Result<Discussion, ServerFnError> {
    let token = std::env::var("GITHUB_TOKEN")
        .map_err(|_| ServerFnError::new("GITHUB_TOKEN not set"))?;

    let client = GitHubClient::new(token);
    
    let data = client
        .get_discussion_detail(&discussion_id)
        .await
        .map_err(|e| ServerFnError::new(format!("GitHub API error: {:?}", e)))?;

    let d = data.node
        .ok_or_else(|| ServerFnError::new("Discussion not found"))?;

    Ok(Discussion {
        id: d.id,
        title: d.title,
        body: d.body,
        body_preview: d.body_text
            .as_ref()
            .map(|t| {
                if t.len() > 200 {
                    format!("{}...", &t[..200])
                } else {
                    t.clone()
                }
            })
            .unwrap_or_default(),
        author: DiscussionAuthor {
            username: d.author.as_ref().map(|a| a.login.clone()).unwrap_or_default(),
            display_name: None,
            avatar_url: d.author.as_ref().map(|a| a.avatar_url.clone()).unwrap_or_default(),
        },
        category: DiscussionCategory {
            id: d.category.id,
            name: d.category.name,
            description: d.category.description,
            emoji: d.category.emoji,
        },
        created_at: d.created_at,
        updated_at: d.updated_at,
        comments_count: d.comments.total_count,
        reactions_count: d.reactions.total_count,
        url: d.url,
        labels: d.labels.nodes.into_iter().map(|l| l.name).collect(),
    })
}
```

### Step 5: mod.rs の更新

`app/src/concepts/discussion/mod.rs` を更新：

```rust
pub mod actions;
pub mod state;
pub mod server;

#[cfg(test)]
mod tests;

pub use actions::*;
pub use state::*;
pub use server::*;
```

---

## 5. テスト項目

### 動作確認手順

1. 環境変数を設定：
```bash
export GITHUB_ORG="your-org"
export GITHUB_TOKEN="ghp_xxxxx"
export GITHUB_DISCUSSIONS_REPO="repo-with-discussions"
```

2. 開発サーバーを起動：
```bash
bun run dev
```

3. ブラウザで動作確認：
   - カテゴリ一覧が取得できることを確認
   - Discussion 一覧が取得できることを確認
   - ページネーションが機能することを確認

---

## 6. 完了条件チェックリスト

- [ ] GraphQL クエリが定義されている
  - [ ] `DISCUSSION_CATEGORIES_QUERY`
  - [ ] `DISCUSSIONS_QUERY`
  - [ ] `DISCUSSION_DETAIL_QUERY`
- [ ] 型定義が完了している
- [ ] GitHubClient にメソッドが追加されている
- [ ] Server Function が実装されている
  - [ ] `get_discussion_categories`
  - [ ] `get_discussions`
  - [ ] `get_discussion_detail`
- [ ] 実際のデータが取得できる
- [ ] ビルドエラーがない

---

## 7. 参照ドキュメント

- GitHub Discussions GraphQL API: https://docs.github.com/en/graphql/reference/objects#discussion
- 既存の GitHub API 実装: `app/src/github/`

---

## 8. 注意点

- **Discussions が有効なリポジトリ**: Organization 内に Discussions が有効なリポジトリが必要
- **レート制限**: GitHub API のレート制限に注意
- **Markdown レンダリング**: `bodyHTML` フィールドを使用するとサーバーサイドでレンダリング済み HTML が取得できる

