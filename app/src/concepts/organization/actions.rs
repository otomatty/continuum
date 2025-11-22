/**
 * Organization Concept - Actions
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   ├─ src/concepts/organization/mod.rs
 *   ├─ src/pages/dashboard/mod.rs
 *   └─ src/concepts/organization/tests.rs
 *
 * Dependencies (External files that this file imports):
 *   └─ ./state.rs
 *
 * Related Documentation:
 *   ├─ Spec: ./organization.spec.md
 *   └─ Plan: docs/03_plans/continuum/legible-architecture-refactoring.md
 */
use super::state::{OrganizationState, OrganizationStats, Period};

/// Initialize mock organization stats for development/testing
pub fn initialize_mock_organization_stats(period: Period) -> OrganizationStats {
    OrganizationStats {
        total_contributors: 45,
        total_repositories: 28,
        external_prs_count: 12,
        total_commits: 1234,
        period,
    }
}

/// Add organization stats to the state
pub fn add_organization_stats(
    state: OrganizationState,
    stats: OrganizationStats,
) -> OrganizationState {
    let mut new_stats = state.stats;
    new_stats.push(stats);
    OrganizationState { stats: new_stats }
}
