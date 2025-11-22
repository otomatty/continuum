use crate::concepts::activity::{ActivityState, ActivityType};
use crate::concepts::organization::Period;
use crate::concepts::ranking::{calculate_score, sort_by_score, RankingEntry, RankingState};
/**
 * Ranking Synchronization
 *
 * DEPENDENCY MAP:
 *
 * Concepts (Concept files that this synchronization imports):
 *   ├─ src/concepts/user/mod.rs
 *   ├─ src/concepts/activity/mod.rs
 *   ├─ src/concepts/ranking/mod.rs
 *   └─ src/concepts/organization/mod.rs (Period enum)
 *
 * Related Documentation:
 *   ├─ Spec: ./ranking.spec.md
 *   ├─ Tests: ./ranking_sync_test.rs
 *   ├─ Related Concepts:
 *   │   ├─ User: src/concepts/user/user.spec.md
 *   │   ├─ Activity: src/concepts/activity/activity.spec.md
 *   │   ├─ Ranking: src/concepts/ranking/ranking.spec.md
 *   │   └─ Organization: src/concepts/organization/organization.spec.md
 *   └─ Plan: docs/03_plans/continuum/legible-architecture-refactoring.md
 */
use crate::concepts::user::UserState;
use chrono::{DateTime, Duration, Utc};

/// Calculate weekly ranking from User and Activity states
/// when: 期間がWeeklyに指定されたら
/// then: UserとActivityから週間ランキングを計算する
pub fn calculate_weekly_ranking(
    user_state: &UserState,
    activity_state: &ActivityState,
) -> Vec<RankingEntry> {
    calculate_ranking_by_period(user_state, activity_state, Period::Weekly)
}

/// Calculate monthly ranking from User and Activity states
/// when: 期間がMonthlyに指定されたら
/// then: UserとActivityから月間ランキングを計算する
pub fn calculate_monthly_ranking(
    user_state: &UserState,
    activity_state: &ActivityState,
) -> Vec<RankingEntry> {
    calculate_ranking_by_period(user_state, activity_state, Period::Monthly)
}

/// Calculate ranking by period (internal helper function)
fn calculate_ranking_by_period(
    user_state: &UserState,
    activity_state: &ActivityState,
    period: Period,
) -> Vec<RankingEntry> {
    let now = Utc::now();
    let cutoff_date = match period {
        Period::Weekly => now - Duration::try_days(7).unwrap(),
        Period::Monthly => now - Duration::try_days(30).unwrap(),
        Period::All => DateTime::parse_from_rfc3339("1970-01-01T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc),
    };

    // Aggregate activities by user
    let mut user_stats: std::collections::HashMap<String, (u32, u32, u32)> =
        std::collections::HashMap::new();

    for activity in &activity_state.activities {
        if activity.created_at >= cutoff_date {
            let stats = user_stats
                .entry(activity.user.username.clone())
                .or_insert((0, 0, 0));
            match activity.activity_type {
                ActivityType::Commit => stats.0 += 1,
                ActivityType::PullRequest => stats.1 += 1,
                ActivityType::Review => stats.2 += 1,
                _ => {}
            }
        }
    }

    // Create ranking entries
    let entries: Vec<RankingEntry> = user_state
        .users
        .iter()
        .filter_map(|user| {
            if let Some((commits, prs, reviews)) = user_stats.get(&user.username) {
                Some(RankingEntry {
                    user: user.clone(),
                    commits: *commits,
                    prs: *prs,
                    reviews: *reviews,
                    score: calculate_score(*commits, *prs, *reviews),
                    rank: 0, // Will be set by sort_by_score
                })
            } else {
                None
            }
        })
        .collect();

    // Sort by score and assign ranks
    let ranking_state = RankingState { entries };
    let sorted_state = sort_by_score(ranking_state);
    sorted_state.entries
}
