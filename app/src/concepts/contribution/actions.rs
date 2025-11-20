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
 *   ├─ ./state.rs
 *   ├─ crate::concepts::user::actions::initialize_mock_users
 *   ├─ crate::concepts::repository::actions::initialize_mock_repositories
 *   └─ crate::concepts::organization::state::Period
 *
 * Related Documentation:
 *   ├─ Spec: ./contribution.spec.md
 *   └─ Plan: docs/03_plans/continuum/legible-architecture-refactoring.md
 */

use super::state::{ContributionGraph, ContributionDay, RepositoryContribution, ContributionState};
use chrono::{Utc, Duration};
use crate::concepts::organization::state::Period;

// Note: モックデータ生成のため、userとrepository conceptのinitialize関数を使用
// これは初期化時のみの依存で、通常のアクションでは他のConceptを参照しない
use crate::concepts::user::actions::{initialize_mock_users, get_user_by_username};
use crate::concepts::repository::actions::initialize_mock_repositories;

/// Initialize mock contribution graph for development/testing
pub fn initialize_mock_contribution_graph(username: &str, period: Period) -> ContributionGraph {
    let user_state = initialize_mock_users();
    let user = get_user_by_username(&user_state, username)
        .unwrap_or_else(|| user_state.users[0].clone());
    
    let days = match period {
        Period::Weekly => 7,
        Period::Monthly => 30,
        Period::All => 365,
    };
    
    let mut data = Vec::new();
    let base_date = Utc::now().date_naive();
    
    for i in 0..days {
        let date = base_date + Duration::try_days(i as i64).expect("i is a valid small integer, try_days should not fail");
        data.push(ContributionDay {
            date,
            commits: (i % 5) as u32,
            prs: (i % 3) as u32,
            reviews: (i % 4) as u32,
        });
    }
    
    ContributionGraph {
        user,
        data,
        period,
    }
}

/// Initialize mock repository contributions for development/testing
pub fn initialize_mock_repository_contributions(username: &str) -> Vec<RepositoryContribution> {
    let repos = initialize_mock_repositories().repositories;
    
    repos.iter().map(|repo| {
        RepositoryContribution {
            repository: repo.clone(),
            commits: (repo.stars / 10) as u32,
            prs: (repo.stars / 20) as u32,
            reviews: (repo.stars / 30) as u32,
            lines_added: (repo.stars * 5) as u32,
            lines_deleted: (repo.stars * 2) as u32,
            percentage: repo.contributors.iter()
                .find(|c| c.user.username == username)
                .map(|c| c.percentage)
                .unwrap_or(0.0),
        }
    }).collect()
}

/// Add a contribution graph to the state
pub fn add_contribution_graph(state: ContributionState, graph: ContributionGraph) -> ContributionState {
    let mut new_graphs = state.graphs;
    new_graphs.push(graph);
    ContributionState {
        graphs: new_graphs,
        ..state
    }
}

