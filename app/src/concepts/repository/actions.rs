/**
 * Repository Concept - Actions
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   ├─ src/concepts/repository/mod.rs
 *   ├─ src/pages/dashboard/mod.rs
 *   ├─ src/pages/portfolio/mod.rs
 *   └─ src/concepts/repository/tests.rs
 *
 * Dependencies (External files that this file imports):
 *   ├─ ./state.rs
 *   └─ crate::concepts::user::actions::initialize_mock_users (モックデータ生成のため)
 *
 * Related Documentation:
 *   ├─ Spec: ./repository.spec.md
 *   └─ Plan: docs/03_plans/continuum/legible-architecture-refactoring.md
 */

use super::state::{Repository, RepositoryState, ContributorStats};
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;

// Note: モックデータ生成のため、user conceptのinitialize_mock_usersを使用
// これは初期化時のみの依存で、通常のアクションでは他のConceptを参照しない
use crate::concepts::user::actions::initialize_mock_users;

// Cache mock repositories to avoid regeneration
static MOCK_REPOSITORIES: Lazy<Vec<Repository>> = Lazy::new(|| {
    let users = initialize_mock_users().users;
    vec![
        Repository {
            name: "awesome-rust".to_string(),
            full_name: "org/awesome-rust".to_string(),
            description: Some("A curated list of awesome Rust resources".to_string()),
            stars: 1250,
            language: Some("Rust".to_string()),
            updated_at: DateTime::parse_from_rfc3339("2024-01-15T10:30:00Z").unwrap().with_timezone(&Utc),
            contributors: vec![
                ContributorStats {
                    user: users[0].clone(),
                    commits: 45,
                    percentage: 35.5,
                },
                ContributorStats {
                    user: users[1].clone(),
                    commits: 32,
                    percentage: 25.2,
                },
            ],
        },
        Repository {
            name: "web-framework".to_string(),
            full_name: "org/web-framework".to_string(),
            description: Some("Modern web framework built with Rust".to_string()),
            stars: 890,
            language: Some("Rust".to_string()),
            updated_at: DateTime::parse_from_rfc3339("2024-01-14T15:20:00Z").unwrap().with_timezone(&Utc),
            contributors: vec![
                ContributorStats {
                    user: users[1].clone(),
                    commits: 78,
                    percentage: 42.3,
                },
                ContributorStats {
                    user: users[2].clone(),
                    commits: 28,
                    percentage: 15.2,
                },
            ],
        },
        Repository {
            name: "cli-tool".to_string(),
            full_name: "org/cli-tool".to_string(),
            description: Some("Command-line tool for developers".to_string()),
            stars: 456,
            language: Some("Rust".to_string()),
            updated_at: DateTime::parse_from_rfc3339("2024-01-13T09:15:00Z").unwrap().with_timezone(&Utc),
            contributors: vec![
                ContributorStats {
                    user: users[0].clone(),
                    commits: 23,
                    percentage: 28.7,
                },
                ContributorStats {
                    user: users[4].clone(),
                    commits: 19,
                    percentage: 23.8,
                },
            ],
        },
        Repository {
            name: "data-processor".to_string(),
            full_name: "org/data-processor".to_string(),
            description: Some("High-performance data processing library".to_string()),
            stars: 234,
            language: Some("Rust".to_string()),
            updated_at: DateTime::parse_from_rfc3339("2024-01-12T14:45:00Z").unwrap().with_timezone(&Utc),
            contributors: vec![
                ContributorStats {
                    user: users[3].clone(),
                    commits: 12,
                    percentage: 18.5,
                },
                ContributorStats {
                    user: users[1].clone(),
                    commits: 8,
                    percentage: 12.3,
                },
            ],
        },
    ]
});

/// Initialize mock repositories for development/testing
pub fn initialize_mock_repositories() -> RepositoryState {
    RepositoryState {
        repositories: MOCK_REPOSITORIES.clone(),
    }
}

/// Add a repository to the state
pub fn add_repository(state: RepositoryState, repository: Repository) -> RepositoryState {
    let mut new_repositories = state.repositories;
    new_repositories.push(repository);
    RepositoryState {
        repositories: new_repositories,
    }
}

/// Find a repository by name
pub fn find_repository_by_name(state: &RepositoryState, name: &str) -> Option<&Repository> {
    state.repositories.iter().find(|r| r.name == name)
}

