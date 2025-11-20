/**
 * Repository Concept - Tests
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   └─ src/concepts/repository/mod.rs (testモジュールとして)
 *
 * Dependencies (External files that this file imports):
 *   ├─ ./state.rs
 *   └─ ./actions.rs
 *
 * Related Documentation:
 *   └─ Spec: ./repository.spec.md
 */

#[cfg(test)]
mod tests {
    use super::super::actions::*;
    use super::super::state::*;
    use chrono::{DateTime, Utc};

    #[test]
    fn test_initialize_mock_repositories() {
        let state = initialize_mock_repositories();
        assert_eq!(state.repositories.len(), 4);
        assert_eq!(state.repositories[0].name, "awesome-rust");
    }

    #[test]
    fn test_add_repository() {
        let state = RepositoryState::default();
        let new_repo = Repository {
            name: "test-repo".to_string(),
            full_name: "org/test-repo".to_string(),
            description: Some("Test repository".to_string()),
            stars: 100,
            language: Some("Rust".to_string()),
            updated_at: DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
            contributors: vec![],
        };

        let result = add_repository(state, new_repo.clone());

        assert_eq!(result.repositories.len(), 1);
        assert_eq!(result.repositories[0].name, "test-repo");
    }

    #[test]
    fn test_find_repository_by_name() {
        let state = initialize_mock_repositories();

        let repo = find_repository_by_name(&state, "awesome-rust");
        assert!(repo.is_some());
        assert_eq!(repo.unwrap().name, "awesome-rust");
    }
}
