/**
 * GitHub Data Concept - Actions
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   ├─ src/concepts/github_data/mod.rs
 *   └─ src/concepts/github_data/tests.rs
 *
 * Dependencies (External files that this file imports):
 *   ├─ ./state.rs
 *   └─ crate::github::client::GitHubClient (SSR only)
 *
 * Related Documentation:
 *   ├─ Spec: ./github_data.spec.md
 *   └─ Plan: docs/03_plans/continuum/tasks/task-05-github-api.md
 */
#[cfg(feature = "ssr")]
use crate::github::client::GitHubClient;

use super::state::{GitHubDataState, OrganizationStats, RepositoryInfo};
use leptos::prelude::*;

/// GitHub Data Stateを初期化
pub fn initialize_github_data_state() -> GitHubDataState {
    GitHubDataState::default()
}

/// Organization Statsを設定
pub fn set_organization_stats(state: GitHubDataState, stats: OrganizationStats) -> GitHubDataState {
    GitHubDataState {
        organization_stats: Some(stats),
        ..state
    }
}

/// Repositoriesを設定
pub fn set_repositories(
    state: GitHubDataState,
    repositories: Vec<RepositoryInfo>,
) -> GitHubDataState {
    GitHubDataState {
        repositories,
        ..state
    }
}

/// Loading状態を設定
pub fn set_loading(state: GitHubDataState, loading: bool) -> GitHubDataState {
    GitHubDataState { loading, ..state }
}

/// Error状態を設定
pub fn set_error(state: GitHubDataState, error: Option<String>) -> GitHubDataState {
    GitHubDataState { error, ..state }
}

/// Organization の統計情報を取得する Server Function
#[server(GetOrganizationStats, "/api/github/stats")]
pub async fn get_organization_stats() -> Result<OrganizationStats, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let org = std::env::var("GITHUB_ORG_NAME")
            .map_err(|_| ServerFnError::new("GITHUB_ORG_NAME not set"))?;

        // OAuth認証済みユーザーのトークンを使用
        // または環境変数からトークンを取得
        // TODO: OAuth認証済みユーザーのトークンをセッションから取得する実装を追加
        let token = std::env::var("GITHUB_TOKEN")
            .map_err(|_| ServerFnError::new("GITHUB_TOKEN not set"))?;

        let client = GitHubClient::new(token)
            .map_err(|e| ServerFnError::new(format!("Failed to create GitHub client: {:?}", e)))?;

        let data = client
            .get_organization_stats(&org)
            .await
            .map_err(|e| ServerFnError::new(format!("GitHub API error: {:?}", e)))?;

        let org_data = data
            .organization
            .ok_or_else(|| ServerFnError::new("Organization not found"))?;

        Ok(OrganizationStats {
            total_contributors: org_data.members_with_role.total_count,
            total_repositories: org_data.repositories.total_count,
            external_prs_count: 0, // TODO: 外部PRの集計ロジックを追加
        })
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new(
            "get_organization_stats is only available on the server",
        ))
    }
}
