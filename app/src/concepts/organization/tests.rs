/**
 * Organization Concept - Tests
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   └─ src/concepts/organization/mod.rs (testモジュールとして)
 *
 * Dependencies (External files that this file imports):
 *   ├─ ./state.rs
 *   └─ ./actions.rs
 *
 * Related Documentation:
 *   └─ Spec: ./organization.spec.md
 */

#[cfg(test)]
mod tests {
    use super::super::actions::*;
    use super::super::state::*;

    #[test]
    fn test_initialize_mock_organization_stats() {
        let stats = initialize_mock_organization_stats(Period::Weekly);
        assert_eq!(stats.total_contributors, 45);
        assert_eq!(stats.total_repositories, 28);
        assert_eq!(stats.period, Period::Weekly);
    }

    #[test]
    fn test_add_organization_stats() {
        let state = OrganizationState::default();
        let stats = OrganizationStats {
            total_contributors: 10,
            total_repositories: 5,
            external_prs_count: 2,
            total_commits: 100,
            period: Period::Monthly,
        };
        
        let result = add_organization_stats(state, stats.clone());
        
        assert_eq!(result.stats.len(), 1);
        assert_eq!(result.stats[0].total_contributors, 10);
    }
}

