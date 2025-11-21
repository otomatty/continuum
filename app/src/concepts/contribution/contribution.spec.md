# Contribution Concept Specification

## Related Files

- Implementation: `src/concepts/contribution/mod.rs`
- State: `src/concepts/contribution/state.rs`
- Actions: `src/concepts/contribution/actions.rs`
- Tests: `src/concepts/contribution/tests.rs`

## Related Documentation

- Issue: (To be created if needed)
- Plan: `docs/03_plans/continuum/legible-architecture-refactoring.md`
- Related Concepts:
  - User: `src/concepts/user/user.spec.md` (ContributionGraph uses User)
  - Repository: `src/concepts/repository/repository.spec.md` (RepositoryContribution uses Repository)
  - Organization: `src/concepts/organization/organization.spec.md` (uses Period enum)

## Requirements

### 責務
- コントリビューション情報の管理
- コントリビューショングラフとリポジトリコントリビューションの管理

### 状態構造
- ContributionState: { graphs: Vec<ContributionGraph>, repository_contributions: Vec<RepositoryContribution> }
- ContributionGraph: { user: User, data: Vec<ContributionDay>, period: Period }
- ContributionDay: { date: NaiveDate, commits: u32, prs: u32, reviews: u32 }
- RepositoryContribution: { repository: Repository, commits: u32, prs: u32, reviews: u32, lines_added: u32, lines_deleted: u32, percentage: f64 }

### アクション
- initialize_mock_contribution_graph: モックコントリビューショングラフを生成
- initialize_mock_repository_contributions: モックリポジトリコントリビューションを生成
- add_contribution_graph: コントリビューショングラフを追加

## Test Cases

### TC-001: initialize_mock_contribution_graph
- Given: username="alice-dev", period=Period::Weekly
- When: initialize_mock_contribution_graph(username, period)を実行
- Then: 7日分のContributionDayを含むContributionGraphが返される

### TC-002: initialize_mock_repository_contributions
- Given: username="alice-dev"
- When: initialize_mock_repository_contributions(username)を実行
- Then: 4つのRepositoryContributionが返される

### TC-003: add_contribution_graph
- Given: 空のContributionState
- When: add_contribution_graph(state, graph)を実行
- Then: graphs配列にgraphが追加される

