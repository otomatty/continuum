# User Concept Specification

## Related Files

- Implementation: `src/concepts/user/mod.rs`
- State: `src/concepts/user/state.rs`
- Actions: `src/concepts/user/actions.rs`
- Tests: `src/concepts/user/tests.rs`

## Related Documentation

- Issue: (To be created if needed)
- Plan: `docs/03_plans/continuum/legible-architecture-refactoring.md`
- Synchronizations:
  - ranking_sync: `src/synchronizations/ranking_sync.rs` (uses User state)

## Requirements

### 責務
- ユーザー情報の管理
- ユーザーの追加・取得・更新

### 状態構造
- UserState: { users: Vec<User>, current_user_id: Option<String> }
- User: { username: String, display_name: String, avatar_url: String, github_url: String, role: UserRole, joined_at: Option<DateTime<Utc>>, left_at: Option<DateTime<Utc>> }
- UserRole: CurrentEmployee | Alumni | ExternalContributor

### アクション
- initialize_mock_users: モックユーザーデータを初期化してUserStateを返す
- add_user: ユーザーを追加
- find_user_by_username: ユーザー名でユーザーを検索
- get_user_by_id: IDでユーザーを取得

## Test Cases

### TC-001: initialize_mock_users
- Given: なし
- When: initialize_mock_users()を実行
- Then: 5人のモックユーザーを含むUserStateが返される

### TC-002: add_user
- Given: 空のUserState
- When: add_user(state, new_user)を実行
- Then: users配列にnew_userが追加される

### TC-003: find_user_by_username
- Given: ユーザーを含むUserState
- When: find_user_by_username(state, "alice-dev")を実行
- Then: usernameが"alice-dev"のユーザーが返される

### TC-004: find_user_by_username_not_found
- Given: ユーザーを含むUserState
- When: find_user_by_username(state, "nonexistent")を実行
- Then: Noneが返される

