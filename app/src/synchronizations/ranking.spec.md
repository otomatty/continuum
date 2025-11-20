# Ranking Synchronization Specification

## Related Files

- Implementation: `src/synchronizations/ranking_sync.rs`
- Tests: `src/synchronizations/ranking_sync_test.rs`

## Related Documentation

- Issue: (To be created if needed)
- Plan: `docs/03_plans/continuum/legible-architecture-refactoring.md`
- Related Concepts:
  - User: `src/concepts/user/user.spec.md`
  - Activity: `src/concepts/activity/activity.spec.md`
  - Ranking: `src/concepts/ranking/ranking.spec.md`
  - Organization: `src/concepts/organization/organization.spec.md` (uses Period enum)

## Requirements

### 責務
- User ConceptとActivity Conceptを連携させてランキングを生成する
- 期間（Period）に応じたランキング計算

### 連携ルール

#### when: 期間が指定されたら
#### then: UserとActivityからランキングを計算する

- User Stateから全ユーザーを取得
- Activity Stateから指定期間内のアクティビティをフィルタリング
- 各ユーザーごとにコミット、PR、レビュー数を集計
- Ranking Conceptのcalculate_scoreでスコアを計算
- スコアでソートしてランキングを生成

### アクション
- calculate_weekly_ranking: 週間ランキングを計算
- calculate_monthly_ranking: 月間ランキングを計算

## Test Cases

### TC-001: calculate_weekly_ranking
- Given: UserStateとActivityState
- When: calculate_weekly_ranking(user_state, activity_state)を実行
- Then: 週間のランキングエントリが返される（スコア順）

### TC-002: calculate_monthly_ranking
- Given: UserStateとActivityState
- When: calculate_monthly_ranking(user_state, activity_state)を実行
- Then: 月間のランキングエントリが返される（スコア順）

