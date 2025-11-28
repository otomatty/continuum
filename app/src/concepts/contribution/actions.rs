/**
 * Contribution Concept - Actions
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   ├─ src/concepts/contribution/mod.rs
 *   ├─ src/pages/portfolio/mod.rs
 *   └─ src/concepts/contribution/tests.rs
 *
 * Dependencies (External files that this file imports):
 *   └─ ./state.rs
 *
 * Related Documentation:
 *   ├─ Spec: ./contribution.spec.md
 *   └─ Plan: docs/03_plans/continuum/legible-architecture-refactoring.md
 *
 * Note: Legible Architecture の「Concept の独立性」原則に従い、
 * 他の Concept を直接参照しない。モックデータは ID 参照のみで構成し、
 * 実際のデータ結合は Synchronization 層で行う
 */
use super::state::{
    ContributionDay, ContributionGraph, ContributionPeriod, ContributionState,
    RepositoryContribution,
};
use chrono::{Duration, Utc};

/// Initialize mock contribution graph for development/testing
/// Note: User は ID のみ参照し、Concept の独立性を維持
pub fn initialize_mock_contribution_graph(
    user_id: &str,
    period: ContributionPeriod,
) -> ContributionGraph {
    let days = match period {
        ContributionPeriod::Weekly => 7,
        ContributionPeriod::Monthly => 30,
        ContributionPeriod::All => 365,
    };

    let mut data = Vec::new();
    let base_date = Utc::now().date_naive();

    for i in 0..days {
        let date = base_date
            + Duration::try_days(i as i64)
                .expect("i is a valid small integer, try_days should not fail");
        data.push(ContributionDay {
            date,
            commits: (i % 5) as u32,
            prs: (i % 3) as u32,
            reviews: (i % 4) as u32,
        });
    }

    ContributionGraph {
        user_id: user_id.to_string(),
        data,
        period,
    }
}

/// Initialize mock repository contributions for development/testing
/// Note: Repository は ID のみ参照し、Concept の独立性を維持
pub fn initialize_mock_repository_contributions(user_id: &str) -> Vec<RepositoryContribution> {
    // モックリポジトリ ID のリスト
    let mock_repos = vec![
        ("awesome-rust", 1200, 15.5),
        ("web-framework", 800, 25.0),
        ("cli-tool", 500, 10.0),
        ("data-processor", 300, 5.0),
    ];

    mock_repos
        .into_iter()
        .map(|(repo_id, stars, percentage)| RepositoryContribution {
            repository_id: repo_id.to_string(),
            commits: stars / 10,
            prs: stars / 20,
            reviews: stars / 30,
            lines_added: stars * 5,
            lines_deleted: stars * 2,
            percentage: if user_id == "alice" {
                percentage
            } else {
                percentage * 0.5
            },
        })
        .collect()
}

/// Add a contribution graph to the state
pub fn add_contribution_graph(
    state: ContributionState,
    graph: ContributionGraph,
) -> ContributionState {
    let mut new_graphs = state.graphs;
    new_graphs.push(graph);
    ContributionState {
        graphs: new_graphs,
        ..state
    }
}

/// Find contribution graph by user ID
pub fn find_contribution_graph_by_user<'a>(
    state: &'a ContributionState,
    user_id: &str,
) -> Option<&'a ContributionGraph> {
    state.graphs.iter().find(|g| g.user_id == user_id)
}

/// Filter repository contributions by repository ID
pub fn filter_by_repository<'a>(
    contributions: &'a [RepositoryContribution],
    repository_id: &str,
) -> Option<&'a RepositoryContribution> {
    contributions
        .iter()
        .find(|c| c.repository_id == repository_id)
}
