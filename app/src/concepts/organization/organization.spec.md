# Organization Concept Specification

## Related Files

- Implementation: `src/concepts/organization/mod.rs`
- State: `src/concepts/organization/state.rs`
- Actions: `src/concepts/organization/actions.rs`
- Tests: `src/concepts/organization/tests.rs`

## Related Documentation

- Issue: (To be created if needed)
- Plan: `docs/03_plans/continuum/legible-architecture-refactoring.md`
- Synchronizations:
  - ranking_sync: `src/synchronizations/ranking_sync.rs` (uses Period enum)

## Requirements

### 責務
- 組織統計情報の管理
- 期間（Period）の定義

### 状態構造
- OrganizationState: { stats: Vec<OrganizationStats> }
- OrganizationStats: { total_contributors: u32, total_repositories: u32, external_prs_count: u32, total_commits: u32, period: Period }
- Period: Weekly | Monthly | All

### アクション
- initialize_mock_organization_stats: モック組織統計データを初期化してOrganizationStatsを返す
- add_organization_stats: 組織統計を追加

## Test Cases

### TC-001: initialize_mock_organization_stats
- Given: Period::Weekly
- When: initialize_mock_organization_stats(period)を実行
- Then: 指定された期間のOrganizationStatsが返される

### TC-002: add_organization_stats
- Given: 空のOrganizationState
- When: add_organization_stats(state, stats)を実行
- Then: stats配列にstatsが追加される

