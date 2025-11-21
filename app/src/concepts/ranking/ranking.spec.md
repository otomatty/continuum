# Ranking Concept Specification

## Related Files

- Implementation: `src/concepts/ranking/mod.rs`
- State: `src/concepts/ranking/state.rs`
- Actions: `src/concepts/ranking/actions.rs`
- Tests: `src/concepts/ranking/tests.rs`

## Related Documentation

- Issue: (To be created if needed)
- Plan: `docs/03_plans/continuum/legible-architecture-refactoring.md`
- Related Concepts:
  - User: `src/concepts/user/user.spec.md` (RankingEntry uses User)
- Synchronizations:
  - ranking_sync: `src/synchronizations/ranking_sync.rs` (generates RankingEntry from User and Activity)

## Requirements

### 責務
- ランキングエントリの管理
- ランキングエントリの操作

### 状態構造
- RankingState: { entries: Vec<RankingEntry> }
- RankingEntry: { user: User, commits: u32, prs: u32, reviews: u32, score: u32, rank: u32 }

### アクション
- add_ranking_entry: ランキングエントリを追加
- calculate_score: コミット、PR、レビュー数からスコアを計算
- sort_by_score: スコアでソート

## Test Cases

### TC-001: add_ranking_entry
- Given: 空のRankingState
- When: add_ranking_entry(state, entry)を実行
- Then: entries配列にentryが追加される

### TC-002: calculate_score
- Given: commits=10, prs=5, reviews=3
- When: calculate_score(commits, prs, reviews)を実行
- Then: 適切なスコアが計算される（例: commits*10 + prs*20 + reviews*15）

### TC-003: sort_by_score
- Given: 複数のランキングエントリを含むRankingState
- When: sort_by_score(state)を実行
- Then: エントリがスコアの降順でソートされる

