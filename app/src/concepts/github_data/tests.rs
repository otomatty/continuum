/**
 * GitHub Data Concept - Tests
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   └─ src/concepts/github_data/mod.rs (testモジュールとして)
 *
 * Dependencies (External files that this file imports):
 *   ├─ ./state.rs
 *   └─ ./actions.rs
 *
 * Related Documentation:
 *   └─ Spec: ./github_data.spec.md
 */
#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use super::super::actions::*;
    use super::super::state::*;

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
        assert_eq!(repo.forks, 20);
        assert_eq!(repo.language, Some("Rust".to_string()));
    }

    #[test]
    fn test_initialize_github_data_state() {
        let state = initialize_github_data_state();
        assert!(state.organization_stats.is_none());
        assert!(state.repositories.is_empty());
        assert!(!state.loading);
        assert!(state.error.is_none());
    }

    #[test]
    fn test_set_organization_stats() {
        let state = GitHubDataState::default();
        let stats = OrganizationStats {
            total_contributors: 10,
            total_repositories: 5,
            external_prs_count: 2,
        };

        let result = set_organization_stats(state, stats.clone());

        assert!(result.organization_stats.is_some());
        assert_eq!(result.organization_stats.unwrap().total_contributors, 10);
    }

    #[test]
    fn test_set_repositories() {
        let state = GitHubDataState::default();
        let repos = vec![RepositoryInfo {
            name: "repo1".to_string(),
            description: None,
            url: "https://github.com/org/repo1".to_string(),
            stars: 10,
            forks: 2,
            language: None,
            updated_at: "2025-01-01T00:00:00Z".to_string(),
        }];

        let result = set_repositories(state, repos.clone());

        assert_eq!(result.repositories.len(), 1);
        assert_eq!(result.repositories[0].name, "repo1");
    }

    #[test]
    fn test_set_loading() {
        let state = GitHubDataState::default();
        let result = set_loading(state, true);

        assert!(result.loading);
    }

    #[test]
    fn test_set_error() {
        let state = GitHubDataState::default();
        let result = set_error(state, Some("Test error".to_string()));

        assert!(result.error.is_some());
        assert_eq!(result.error.unwrap(), "Test error");
    }
}

