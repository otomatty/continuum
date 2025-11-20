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
mod tests {
    use super::super::ranking_sync::*;
    use crate::concepts::user::{UserState, User, UserRole};
    use crate::concepts::activity::{ActivityState, Activity, ActivityType};
    use crate::concepts::organization::Period;
    use chrono::{DateTime, Utc};

    fn create_test_user(username: &str) -> User {
        User {
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
                create_test_user("user1"),
                create_test_user("user2"),
            ],
            current_user_id: None,
        };

        let now = Utc::now();
        let activity_state = ActivityState {
            activities: vec![
                Activity {
                    id: "1".to_string(),
                    activity_type: ActivityType::Commit,
                    user: user_state.users[0].clone(),
                    repository: crate::concepts::repository::actions::initialize_mock_repositories().repositories[0].clone(),
                    title: "Test commit".to_string(),
                    created_at: now - chrono::Duration::try_days(1).unwrap(),
                    url: "https://github.com/test".to_string(),
                },
                Activity {
                    id: "2".to_string(),
                    activity_type: ActivityType::PullRequest,
                    user: user_state.users[1].clone(),
                    repository: crate::concepts::repository::actions::initialize_mock_repositories().repositories[0].clone(),
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
            assert!(ranking[i-1].score >= ranking[i].score);
        }
    }

    #[test]
    fn test_calculate_monthly_ranking() {
        let user_state = UserState {
            users: vec![create_test_user("user1")],
            current_user_id: None,
        };

        let activity_state = ActivityState {
            activities: vec![],
        };

        let ranking = calculate_monthly_ranking(&user_state, &activity_state);
        
        // With no activities, ranking should be empty
        assert!(ranking.is_empty());
    }
}

