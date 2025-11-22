/**
 * Ranking Concept - Actions
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   ├─ src/concepts/ranking/mod.rs
 *   └─ src/concepts/ranking/tests.rs
 *
 * Dependencies (External files that this file imports):
 *   └─ ./state.rs
 *
 * Related Documentation:
 *   ├─ Spec: ./ranking.spec.md
 *   └─ Plan: docs/03_plans/continuum/legible-architecture-refactoring.md
 */
use super::state::{RankingEntry, RankingState};

/// Add a ranking entry to the state
pub fn add_ranking_entry(state: RankingState, entry: RankingEntry) -> RankingState {
    let mut new_entries = state.entries;
    new_entries.push(entry);
    RankingState {
        entries: new_entries,
    }
}

/// Calculate score from commits, PRs, and reviews
/// Score formula: commits * 10 + prs * 20 + reviews * 15
pub fn calculate_score(commits: u32, prs: u32, reviews: u32) -> u32 {
    commits * 10 + prs * 20 + reviews * 15
}

/// Sort ranking entries by score in descending order
pub fn sort_by_score(mut state: RankingState) -> RankingState {
    state.entries.sort_by(|a, b| b.score.cmp(&a.score));
    // Update ranks after sorting
    for (index, entry) in state.entries.iter_mut().enumerate() {
        entry.rank = (index + 1) as u32;
    }
    state
}
