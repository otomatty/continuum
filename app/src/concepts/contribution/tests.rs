/**
 * Contribution Concept - Tests
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   └─ src/concepts/contribution/mod.rs (testモジュールとして)
 *
 * Dependencies (External files that this file imports):
 *   ├─ ./state.rs
 *   └─ ./actions.rs
 *
 * Related Documentation:
 *   └─ Spec: ./contribution.spec.md
 */
#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use super::super::actions::*;
    use super::super::state::*;

    #[test]
    fn test_initialize_mock_contribution_graph() {
        let graph = initialize_mock_contribution_graph("alice-dev", ContributionPeriod::Weekly);
        assert_eq!(graph.data.len(), 7);
        assert_eq!(graph.user_id, "alice-dev");
        assert_eq!(graph.period, ContributionPeriod::Weekly);
    }

    #[test]
    fn test_initialize_mock_repository_contributions() {
        let contributions = initialize_mock_repository_contributions("alice-dev");
        assert_eq!(contributions.len(), 4);
    }

    #[test]
    fn test_add_contribution_graph() {
        let state = ContributionState::default();
        let graph = ContributionGraph {
            user_id: "user-test".to_string(),
            data: vec![],
            period: ContributionPeriod::Weekly,
        };

        let result = add_contribution_graph(state, graph.clone());

        assert_eq!(result.graphs.len(), 1);
        assert_eq!(result.graphs[0].user_id, "user-test");
    }
}
