/**
 * User Concept - Actions
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   ├─ src/concepts/user/mod.rs
 *   ├─ src/pages/dashboard/mod.rs
 *   ├─ src/pages/portfolio/mod.rs
 *   └─ src/concepts/user/tests.rs
 *
 * Dependencies (External files that this file imports):
 *   └─ ./state.rs
 *
 * Related Documentation:
 *   ├─ Spec: ./user.spec.md
 *   └─ Plan: docs/03_plans/continuum/legible-architecture-refactoring.md
 */
use super::state::{User, UserRole, UserState};
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;

// Cache mock users to avoid regeneration
static MOCK_USERS: Lazy<Vec<User>> = Lazy::new(|| {
    vec![
        User {
            id: "user-alice".to_string(),
            username: "alice-dev".to_string(),
            display_name: "Alice Developer".to_string(),
            avatar_url: "https://api.dicebear.com/7.x/avataaars/svg?seed=alice".to_string(),
            github_url: "https://github.com/alice-dev".to_string(),
            role: UserRole::CurrentEmployee,
            joined_at: Some(
                DateTime::parse_from_rfc3339("2022-01-15T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
            left_at: None,
        },
        User {
            id: "user-bob".to_string(),
            username: "bob-engineer".to_string(),
            display_name: "Bob Engineer".to_string(),
            avatar_url: "https://api.dicebear.com/7.x/avataaars/svg?seed=bob".to_string(),
            github_url: "https://github.com/bob-engineer".to_string(),
            role: UserRole::CurrentEmployee,
            joined_at: Some(
                DateTime::parse_from_rfc3339("2021-06-01T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
            left_at: None,
        },
        User {
            id: "user-charlie".to_string(),
            username: "charlie-coder".to_string(),
            display_name: "Charlie Coder".to_string(),
            avatar_url: "https://api.dicebear.com/7.x/avataaars/svg?seed=charlie".to_string(),
            github_url: "https://github.com/charlie-coder".to_string(),
            role: UserRole::Alumni,
            joined_at: Some(
                DateTime::parse_from_rfc3339("2020-03-10T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
            left_at: Some(
                DateTime::parse_from_rfc3339("2023-12-31T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
        },
        User {
            id: "user-diana".to_string(),
            username: "diana-hacker".to_string(),
            display_name: "Diana Hacker".to_string(),
            avatar_url: "https://api.dicebear.com/7.x/avataaars/svg?seed=diana".to_string(),
            github_url: "https://github.com/diana-hacker".to_string(),
            role: UserRole::ExternalContributor,
            joined_at: None,
            left_at: None,
        },
        User {
            id: "user-eve".to_string(),
            username: "eve-maker".to_string(),
            display_name: "Eve Maker".to_string(),
            avatar_url: "https://api.dicebear.com/7.x/avataaars/svg?seed=eve".to_string(),
            github_url: "https://github.com/eve-maker".to_string(),
            role: UserRole::CurrentEmployee,
            joined_at: Some(
                DateTime::parse_from_rfc3339("2023-02-20T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
            left_at: None,
        },
    ]
});

/// Initialize mock users for development/testing
pub fn initialize_mock_users() -> UserState {
    UserState {
        users: MOCK_USERS.clone(),
        current_user_id: None,
    }
}

/// Add a user to the state
pub fn add_user(state: UserState, user: User) -> UserState {
    let mut new_users = state.users;
    new_users.push(user);
    UserState {
        users: new_users,
        ..state
    }
}

/// Find a user by username
pub fn find_user_by_username<'a>(state: &'a UserState, username: &'a str) -> Option<&'a User> {
    state.users.iter().find(|u| u.username == username)
}

/// Get a user by username (cloned)
pub fn get_user_by_username(state: &UserState, username: &str) -> Option<User> {
    state.users.iter().find(|u| u.username == username).cloned()
}
