/**
 * User Concept - Tests
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   └─ src/concepts/user/mod.rs (testモジュールとして)
 *
 * Dependencies (External files that this file imports):
 *   ├─ ./state.rs
 *   └─ ./actions.rs
 *
 * Related Documentation:
 *   └─ Spec: ./user.spec.md
 */
#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use super::super::actions::*;
    use super::super::state::*;

    #[test]
    fn test_initialize_mock_users() {
        let state = initialize_mock_users();
        assert_eq!(state.users.len(), 5);
        assert_eq!(state.users[0].username, "alice-dev");
        assert_eq!(state.users[1].username, "bob-engineer");
    }

    #[test]
    fn test_add_user() {
        let state = UserState::default();
        let new_user = User {
            username: "test-user".to_string(),
            display_name: "Test User".to_string(),
            avatar_url: "https://example.com/avatar.png".to_string(),
            github_url: "https://github.com/test-user".to_string(),
            role: UserRole::CurrentEmployee,
            joined_at: None,
            left_at: None,
        };

        let result = add_user(state, new_user.clone());

        assert_eq!(result.users.len(), 1);
        assert_eq!(result.users[0].username, "test-user");
    }

    #[test]
    fn test_find_user_by_username() {
        let state = initialize_mock_users();

        let user = find_user_by_username(&state, "alice-dev");
        assert!(user.is_some());
        assert_eq!(user.unwrap().username, "alice-dev");
    }

    #[test]
    fn test_find_user_by_username_not_found() {
        let state = initialize_mock_users();

        let user = find_user_by_username(&state, "nonexistent");
        assert!(user.is_none());
    }

    #[test]
    fn test_get_user_by_username() {
        let state = initialize_mock_users();

        let user = get_user_by_username(&state, "bob-engineer");
        assert!(user.is_some());
        assert_eq!(user.unwrap().username, "bob-engineer");
    }
}
