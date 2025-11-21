# Repository Concept Specification

## Related Files

- Implementation: `src/concepts/repository/mod.rs`
- State: `src/concepts/repository/state.rs`
- Actions: `src/concepts/repository/actions.rs`
- Tests: `src/concepts/repository/tests.rs`

## Related Documentation

- Issue: (To be created if needed)
- Plan: `docs/03_plans/continuum/legible-architecture-refactoring.md`
- Related Concepts:
  - User: `src/concepts/user/user.spec.md` (Repository uses User via ContributorStats)
- Synchronizations:
  - ranking_sync: `src/synchronizations/ranking_sync.rs` (uses Repository state)

## Requirements

### 責務
- リポジトリ情報の管理
- リポジトリの追加・取得・更新

### 状態構造
- RepositoryState: { repositories: Vec<Repository> }
- Repository: { name: String, full_name: String, description: Option<String>, stars: u32, language: Option<String>, updated_at: DateTime<Utc>, contributors: Vec<ContributorStats> }
- ContributorStats: { user: User (from user concept), commits: u32, percentage: f64 }

### アクション
- initialize_mock_repositories: モックリポジトリデータを初期化してRepositoryStateを返す
- add_repository: リポジトリを追加
- find_repository_by_name: リポジトリ名でリポジトリを検索

## Test Cases

### TC-001: initialize_mock_repositories
- Given: なし
- When: initialize_mock_repositories()を実行
- Then: 4つのモックリポジトリを含むRepositoryStateが返される

### TC-002: add_repository
- Given: 空のRepositoryState
- When: add_repository(state, new_repo)を実行
- Then: repositories配列にnew_repoが追加される

### TC-003: find_repository_by_name
- Given: リポジトリを含むRepositoryState
- When: find_repository_by_name(state, "awesome-rust")を実行
- Then: nameが"awesome-rust"のリポジトリが返される

