/**
 * Discussion Concept - Server Functions
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   ├─ src/concepts/discussion/mod.rs
 *   ├─ src/pages/knowledge/mod.rs
 *   └─ src/pages/knowledge_detail/mod.rs
 *
 * Dependencies (External files that this file imports):
 *   ├─ src/github/client.rs (ssr only)
 *   └─ ./state.rs
 *
 * Related Documentation:
 *   ├─ Spec: ./discussion.spec.md
 *   └─ Task: docs/03_plans/continuum/tasks/task-11-github-discussions-api.md
 */
use super::state::*;
use leptos::prelude::*;

/// Discussion カテゴリ一覧を取得
#[server(GetDiscussionCategories, "/api/discussions/categories")]
pub async fn get_discussion_categories(
    repo_name: String,
) -> Result<Vec<DiscussionCategory>, ServerFnError> {
    use crate::github::client::GitHubClient;

    let org = std::env::var("GITHUB_ORG_NAME")
        .map_err(|_| ServerFnError::new("GITHUB_ORG_NAME not set"))?;

    let token =
        std::env::var("GITHUB_TOKEN").map_err(|_| ServerFnError::new("GITHUB_TOKEN not set"))?;

    let client = GitHubClient::new(token)
        .map_err(|e| ServerFnError::new(format!("Failed to create GitHub client: {:?}", e)))?;

    let data = client
        .get_discussion_categories(&org, &repo_name)
        .await
        .map_err(|e| ServerFnError::new(format!("GitHub API error: {:?}", e)))?;

    let repo = data
        .repository
        .ok_or_else(|| ServerFnError::new("Repository not found"))?;

    let categories = repo
        .discussion_categories
        .nodes
        .into_iter()
        .map(|c| DiscussionCategory {
            id: c.id,
            name: c.name,
            description: c.description,
            emoji: c.emoji,
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
    use crate::github::client::GitHubClient;

    let org = std::env::var("GITHUB_ORG_NAME")
        .map_err(|_| ServerFnError::new("GITHUB_ORG_NAME not set"))?;

    let token =
        std::env::var("GITHUB_TOKEN").map_err(|_| ServerFnError::new("GITHUB_TOKEN not set"))?;

    let client = GitHubClient::new(token)
        .map_err(|e| ServerFnError::new(format!("Failed to create GitHub client: {:?}", e)))?;

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

    let repo = data
        .repository
        .ok_or_else(|| ServerFnError::new("Repository not found"))?;

    let discussions = repo
        .discussions
        .nodes
        .into_iter()
        .map(|d| {
            let body_preview = d
                .body_text
                .as_ref()
                .map(|t| {
                    if t.len() > 200 {
                        format!("{}...", &t.chars().take(200).collect::<String>())
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
                    username: d
                        .author
                        .as_ref()
                        .map(|a| a.login.clone())
                        .unwrap_or_default(),
                    display_name: None,
                    avatar_url: d
                        .author
                        .as_ref()
                        .map(|a| a.avatar_url.clone())
                        .unwrap_or_default(),
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
pub async fn get_discussion_detail(discussion_id: String) -> Result<Discussion, ServerFnError> {
    use crate::github::client::GitHubClient;

    let token =
        std::env::var("GITHUB_TOKEN").map_err(|_| ServerFnError::new("GITHUB_TOKEN not set"))?;

    let client = GitHubClient::new(token)
        .map_err(|e| ServerFnError::new(format!("Failed to create GitHub client: {:?}", e)))?;

    let data = client
        .get_discussion_detail(&discussion_id)
        .await
        .map_err(|e| ServerFnError::new(format!("GitHub API error: {:?}", e)))?;

    let d = data
        .node
        .ok_or_else(|| ServerFnError::new("Discussion not found"))?;

    let body_preview = d
        .body_text
        .as_ref()
        .map(|t| {
            if t.len() > 200 {
                format!("{}...", &t.chars().take(200).collect::<String>())
            } else {
                t.clone()
            }
        })
        .unwrap_or_default();

    Ok(Discussion {
        id: d.id,
        title: d.title,
        body: d.body,
        body_preview,
        author: DiscussionAuthor {
            username: d
                .author
                .as_ref()
                .map(|a| a.login.clone())
                .unwrap_or_default(),
            display_name: None,
            avatar_url: d
                .author
                .as_ref()
                .map(|a| a.avatar_url.clone())
                .unwrap_or_default(),
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
