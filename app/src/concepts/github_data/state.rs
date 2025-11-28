/**
 * GitHub Data Concept - State
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   ├─ src/concepts/github_data/mod.rs
 *   ├─ src/concepts/github_data/actions.rs
 *   └─ src/concepts/github_data/tests.rs
 *
 * Dependencies (External files that this file imports):
 *   └─ (なし - 独立したConcept)
 *
 * Related Documentation:
 *   ├─ Spec: ./github_data.spec.md
 *   └─ Plan: docs/03_plans/continuum/tasks/task-05-github-api.md
 */
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
    pub total_contributors: i64,
    pub total_repositories: i64,
    pub external_prs_count: i64,
}

/// リポジトリ情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryInfo {
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub stars: i64,
    pub forks: i64,
    pub language: Option<String>,
    pub updated_at: String,
}
