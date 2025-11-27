# GitHub Data Concept Specification

## Related Files

- Implementation: `src/concepts/github_data/mod.rs`
- State: `src/concepts/github_data/state.rs`
- Actions: `src/concepts/github_data/actions.rs`
- Tests: `src/concepts/github_data/tests.rs`

## Related Documentation

- Issue: (To be created if needed)
- Plan: `docs/03_plans/continuum/tasks/task-05-github-api.md`
- Synchronizations: (To be created if needed)

## Requirements

### 責務
- GitHub APIから取得したデータの状態管理
- 組織統計情報の管理
- リポジトリ情報の管理
- データ取得のローディング状態とエラー状態の管理

### 状態構造
- GitHubDataState: { organization_stats: Option<OrganizationStats>, repositories: Vec<RepositoryInfo>, loading: bool, error: Option<String> }
- OrganizationStats: { total_contributors: i32, total_repositories: i32, external_prs_count: i32 }
- RepositoryInfo: { name: String, description: Option<String>, url: String, stars: i32, forks: i32, language: Option<String>, updated_at: String }

### アクション
- initialize_github_data_state: GitHub Data Stateを初期化
- set_organization_stats: Organization Statsを設定
- set_repositories: Repositoriesを設定
- set_loading: Loading状態を設定
- set_error: Error状態を設定
- get_organization_stats: Server Function - Organization の統計情報を取得（GitHub API経由）

## Test Cases

### TC-001: organization_stats_default
- Given: デフォルトのGitHubDataState
- When: GitHubDataState::default()を実行
- Then: organization_statsはNone、repositoriesは空、loadingはfalse、errorはNone

### TC-002: repository_info_creation
- Given: RepositoryInfoの作成
- When: RepositoryInfo構造体を作成
- Then: 指定された値でRepositoryInfoが作成される

### TC-003: initialize_github_data_state
- Given: なし
- When: initialize_github_data_state()を実行
- Then: デフォルトのGitHubDataStateが返される

### TC-004: set_organization_stats
- Given: 空のGitHubDataStateとOrganizationStats
- When: set_organization_stats(state, stats)を実行
- Then: organization_statsにstatsが設定される

### TC-005: set_repositories
- Given: 空のGitHubDataStateとRepositoryInfoのベクター
- When: set_repositories(state, repos)を実行
- Then: repositoriesにreposが設定される

### TC-006: set_loading
- Given: GitHubDataState
- When: set_loading(state, true)を実行
- Then: loadingがtrueに設定される

### TC-007: set_error
- Given: GitHubDataState
- When: set_error(state, Some("error message"))を実行
- Then: errorに"error message"が設定される

