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
 *   └─ ./state.rs
 *
 * Related Documentation:
 *   ├─ Spec: ./activity.spec.md
 *   └─ Plan: docs/03_plans/continuum/legible-architecture-refactoring.md
 *
 * Note: Legible Architecture の「Concept の独立性」原則に従い、
 * 他の Concept を直接参照しない。モックデータは ID 参照のみで構成し、
 * 実際のデータ結合は Synchronization 層で行う
 */
use super::state::{Activity, ActivityState, ActivityType};
use chrono::{DateTime, Utc};

/// Initialize mock activities for development/testing
/// Note: User/Repository は ID のみ参照し、Concept の独立性を維持
pub fn initialize_mock_activities() -> ActivityState {
    ActivityState {
        activities: vec![
            Activity {
                id: "1".to_string(),
                activity_type: ActivityType::Commit,
                user_id: "alice".to_string(),
                repository_id: "awesome-rust".to_string(),
                title: "Add new feature for async processing".to_string(),
                created_at: DateTime::parse_from_rfc3339("2024-01-15T10:30:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
                url: "https://github.com/org/awesome-rust/commit/abc123".to_string(),
            },
            Activity {
                id: "2".to_string(),
                activity_type: ActivityType::PullRequest,
                user_id: "bob".to_string(),
                repository_id: "web-framework".to_string(),
                title: "Improve error handling".to_string(),
                created_at: DateTime::parse_from_rfc3339("2024-01-14T15:20:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
                url: "https://github.com/org/web-framework/pull/42".to_string(),
            },
            Activity {
                id: "3".to_string(),
                activity_type: ActivityType::Review,
                user_id: "charlie".to_string(),
                repository_id: "awesome-rust".to_string(),
                title: "Reviewed PR #123".to_string(),
                created_at: DateTime::parse_from_rfc3339("2024-01-14T11:15:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
                url: "https://github.com/org/awesome-rust/pull/123".to_string(),
            },
            Activity {
                id: "4".to_string(),
                activity_type: ActivityType::Commit,
                user_id: "david".to_string(),
                repository_id: "data-processor".to_string(),
                title: "Fix memory leak in data processor".to_string(),
                created_at: DateTime::parse_from_rfc3339("2024-01-13T09:15:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
                url: "https://github.com/org/data-processor/commit/def456".to_string(),
            },
            Activity {
                id: "5".to_string(),
                activity_type: ActivityType::PullRequest,
                user_id: "eve".to_string(),
                repository_id: "cli-tool".to_string(),
                title: "Add new CLI command".to_string(),
                created_at: DateTime::parse_from_rfc3339("2024-01-12T14:45:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
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

/// Filter activities by user ID
pub fn filter_by_user(state: &ActivityState, user_id: &str) -> Vec<Activity> {
    state
        .activities
        .iter()
        .filter(|a| a.user_id == user_id)
        .cloned()
        .collect()
}

/// Filter activities by repository ID
pub fn filter_by_repository(state: &ActivityState, repository_id: &str) -> Vec<Activity> {
    state
        .activities
        .iter()
        .filter(|a| a.repository_id == repository_id)
        .cloned()
        .collect()
}
