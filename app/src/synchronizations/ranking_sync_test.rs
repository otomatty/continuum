/**
 * Ranking Synchronization - Tests
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   └─ src/synchronizations/mod.rs (testモジュールとして)
 *
 * Dependencies (External files that this file imports):
 *   ├─ ./ranking_sync.rs
 *   ├─ crate::concepts::user
 *   ├─ crate::concepts::activity
 *   └─ crate::concepts::organization::Period
 *
 * Related Documentation:
 *   └─ Spec: ./ranking.spec.md
 */
#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use super::super::ranking_sync::*;
    use crate::concepts::activity::{Activity, ActivityState, ActivityType};
    use crate::concepts::user::{User, UserRole, UserState};
    use chrono::Utc;

    fn create_test_user(id: &str, username: &str) -> User {
        User {
            id: id.to_string(),
            username: username.to_string(),
            display_name: format!("{} Display", username),
            avatar_url: format!("https://example.com/{}", username),
            github_url: format!("https://github.com/{}", username),
            role: UserRole::CurrentEmployee,
            joined_at: None,
            left_at: None,
        }
    }

    #[test]
    fn test_calculate_weekly_ranking() {
        let user_state = UserState {
            users: vec![
                create_test_user("user-1", "user1"),
                create_test_user("user-2", "user2"),
            ],
            current_user_id: None,
        };

        let now = Utc::now();
        let activity_state = ActivityState {
            activities: vec![
                Activity {
                    id: "1".to_string(),
                    activity_type: ActivityType::Commit,
                    user_id: "user-1".to_string(),
                    repository_id: "repo-1".to_string(),
                    title: "Test commit".to_string(),
                    created_at: now - chrono::Duration::try_days(1).unwrap(),
                    url: "https://github.com/test".to_string(),
                },
                Activity {
                    id: "2".to_string(),
                    activity_type: ActivityType::PullRequest,
                    user_id: "user-2".to_string(),
                    repository_id: "repo-1".to_string(),
                    title: "Test PR".to_string(),
                    created_at: now - chrono::Duration::try_days(2).unwrap(),
                    url: "https://github.com/test".to_string(),
                },
            ],
        };

        let ranking = calculate_weekly_ranking(&user_state, &activity_state);

        assert!(!ranking.is_empty());
        // Check that ranking is sorted by score (descending)
        for i in 1..ranking.len() {
            assert!(ranking[i - 1].score >= ranking[i].score);
        }
    }

    #[test]
    fn test_calculate_monthly_ranking() {
        let user_state = UserState {
            users: vec![create_test_user("user-1", "user1")],
            current_user_id: None,
        };

        let activity_state = ActivityState { activities: vec![] };

        let ranking = calculate_monthly_ranking(&user_state, &activity_state);

        // With no activities, ranking should be empty
        assert!(ranking.is_empty());
    }
}
