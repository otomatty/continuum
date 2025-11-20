# Activity Concept Specification

## Related Files

- Implementation: `src/concepts/activity/mod.rs`
- State: `src/concepts/activity/state.rs`
- Actions: `src/concepts/activity/actions.rs`
- Tests: `src/concepts/activity/tests.rs`

## Related Documentation

- Issue: (To be created if needed)
- Plan: `docs/03_plans/continuum/legible-architecture-refactoring.md`
- Related Concepts:
  - User: `src/concepts/user/user.spec.md` (Activity uses User)
  - Repository: `src/concepts/repository/repository.spec.md` (Activity uses Repository)
- Synchronizations:
  - ranking_sync: `src/synchronizations/ranking_sync.rs` (uses Activity state)

## Requirements

### 責務
- アクティビティ情報の管理
- アクティビティの追加・取得

### 状態構造
- ActivityState: { activities: Vec<Activity> }
- Activity: { id: String, activity_type: ActivityType, user: User, repository: Repository, title: String, created_at: DateTime<Utc>, url: String }
- ActivityType: Commit | PullRequest | Review | Issue | Discussion

### アクション
- initialize_mock_activities: モックアクティビティデータを初期化してActivityStateを返す
- add_activity: アクティビティを追加
- find_activity_by_id: IDでアクティビティを検索

## Test Cases

### TC-001: initialize_mock_activities
- Given: なし
- When: initialize_mock_activities()を実行
- Then: 5つのモックアクティビティを含むActivityStateが返される

### TC-002: add_activity
- Given: 空のActivityState
- When: add_activity(state, new_activity)を実行
- Then: activities配列にnew_activityが追加される

### TC-003: find_activity_by_id
- Given: アクティビティを含むActivityState
- When: find_activity_by_id(state, "1")を実行
- Then: idが"1"のアクティビティが返される

