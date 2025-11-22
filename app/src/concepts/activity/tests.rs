/**
 * Activity Concept - Tests
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   └─ src/concepts/activity/mod.rs (testモジュールとして)
 *
 * Dependencies (External files that this file imports):
 *   ├─ ./state.rs
 *   └─ ./actions.rs
 *
 * Related Documentation:
 *   └─ Spec: ./activity.spec.md
 */
#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use super::super::actions::*;
    use super::super::state::*;
    use crate::concepts::repository::state::Repository;
    use crate::concepts::user::state::{User, UserRole};
    use chrono::{DateTime, Utc};

    #[test]
    fn test_initialize_mock_activities() {
        let state = initialize_mock_activities();
        assert_eq!(state.activities.len(), 5);
        assert_eq!(state.activities[0].id, "1");
    }

    #[test]
    fn test_add_activity() {
        let state = ActivityState::default();
        let user = User {
            username: "test-user".to_string(),
            display_name: "Test User".to_string(),
            avatar_url: "https://example.com/avatar.png".to_string(),
            github_url: "https://github.com/test-user".to_string(),
            role: UserRole::CurrentEmployee,
            joined_at: None,
            left_at: None,
        };
        let repo = Repository {
            name: "test-repo".to_string(),
            full_name: "org/test-repo".to_string(),
            description: None,
            stars: 0,
            language: None,
            updated_at: DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
            contributors: vec![],
        };
        let new_activity = Activity {
            id: "test-1".to_string(),
            activity_type: ActivityType::Commit,
            user: user.clone(),
            repository: repo.clone(),
            title: "Test commit".to_string(),
            created_at: DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
            url: "https://github.com/test".to_string(),
        };

        let result = add_activity(state, new_activity.clone());

        assert_eq!(result.activities.len(), 1);
        assert_eq!(result.activities[0].id, "test-1");
    }

    #[test]
    fn test_find_activity_by_id() {
        let state = initialize_mock_activities();

        let activity = find_activity_by_id(&state, "1");
        assert!(activity.is_some());
        assert_eq!(activity.unwrap().id, "1");
    }
}
