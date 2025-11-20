/**
 * Activity Concept - Actions
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   ├─ src/concepts/activity/mod.rs
 *   ├─ src/pages/dashboard/mod.rs
 *   └─ src/concepts/activity/tests.rs
 *
 * Dependencies (External files that this file imports):
 *   ├─ ./state.rs
 *   ├─ crate::concepts::user::actions::initialize_mock_users
 *   └─ crate::concepts::repository::actions::initialize_mock_repositories
 *
 * Related Documentation:
 *   ├─ Spec: ./activity.spec.md
 *   └─ Plan: docs/03_plans/continuum/legible-architecture-refactoring.md
 */

use super::state::{Activity, ActivityState, ActivityType};
use chrono::{DateTime, Utc};

// Note: モックデータ生成のため、userとrepository conceptのinitialize関数を使用
// これは初期化時のみの依存で、通常のアクションでは他のConceptを参照しない
use crate::concepts::user::actions::initialize_mock_users;
use crate::concepts::repository::actions::initialize_mock_repositories;

/// Initialize mock activities for development/testing
pub fn initialize_mock_activities() -> ActivityState {
    let users = initialize_mock_users().users;
    let repos = initialize_mock_repositories().repositories;
    
    ActivityState {
        activities: vec![
            Activity {
                id: "1".to_string(),
                activity_type: ActivityType::Commit,
                user: users[0].clone(),
                repository: repos[0].clone(),
                title: "Add new feature for async processing".to_string(),
                created_at: DateTime::parse_from_rfc3339("2024-01-15T10:30:00Z").unwrap().with_timezone(&Utc),
                url: "https://github.com/org/awesome-rust/commit/abc123".to_string(),
            },
            Activity {
                id: "2".to_string(),
                activity_type: ActivityType::PullRequest,
                user: users[1].clone(),
                repository: repos[1].clone(),
                title: "Improve error handling".to_string(),
                created_at: DateTime::parse_from_rfc3339("2024-01-14T15:20:00Z").unwrap().with_timezone(&Utc),
                url: "https://github.com/org/web-framework/pull/42".to_string(),
            },
            Activity {
                id: "3".to_string(),
                activity_type: ActivityType::Review,
                user: users[2].clone(),
                repository: repos[0].clone(),
                title: "Reviewed PR #123".to_string(),
                created_at: DateTime::parse_from_rfc3339("2024-01-14T11:15:00Z").unwrap().with_timezone(&Utc),
                url: "https://github.com/org/awesome-rust/pull/123".to_string(),
            },
            Activity {
                id: "4".to_string(),
                activity_type: ActivityType::Commit,
                user: users[3].clone(),
                repository: repos[3].clone(),
                title: "Fix memory leak in data processor".to_string(),
                created_at: DateTime::parse_from_rfc3339("2024-01-13T09:15:00Z").unwrap().with_timezone(&Utc),
                url: "https://github.com/org/data-processor/commit/def456".to_string(),
            },
            Activity {
                id: "5".to_string(),
                activity_type: ActivityType::PullRequest,
                user: users[4].clone(),
                repository: repos[2].clone(),
                title: "Add new CLI command".to_string(),
                created_at: DateTime::parse_from_rfc3339("2024-01-12T14:45:00Z").unwrap().with_timezone(&Utc),
                url: "https://github.com/org/cli-tool/pull/56".to_string(),
            },
        ],
    }
}

/// Add an activity to the state
pub fn add_activity(state: ActivityState, activity: Activity) -> ActivityState {
    let mut new_activities = state.activities;
    new_activities.push(activity);
    ActivityState {
        activities: new_activities,
    }
}

/// Find an activity by ID
pub fn find_activity_by_id<'a>(state: &'a ActivityState, id: &'a str) -> Option<&'a Activity> {
    state.activities.iter().find(|a| a.id == id)
}

