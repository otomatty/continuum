# Filter Concept Specification

## Related Files

- Implementation: `src/concepts/filter/mod.rs`
- State: `src/concepts/filter/state.rs`
- Actions: `src/concepts/filter/actions.rs`
- Tests: `src/concepts/filter/tests.rs`

## Related Documentation

- Issue: (To be created if needed)
- Plan: `docs/03_plans/continuum/tasks/task-06-search-filter-concepts.md`
- Synchronizations: (To be created if needed)

## Requirements

### 責務
- フィルター条件の状態管理（言語、ステータス、カテゴリ）
- ソート設定の管理
- フィルター条件に基づくマッチングロジックの提供

### 状態構造
- FilterState: { languages: HashSet<String>, statuses: HashSet<String>, categories: HashSet<String>, sort_by: SortOption, sort_direction: SortDirection }
- SortOption: Name | UpdatedAt | Stars | Contributions | CreatedAt
- SortDirection: Ascending | Descending

### アクション
- add_language: 言語フィルターを追加
- remove_language: 言語フィルターを削除
- toggle_language: 言語フィルターをトグル
- set_statuses: ステータスフィルターを設定
- set_categories: カテゴリフィルターを設定
- set_sort: ソートオプションと方向を設定
- clear_all: 全フィルターをクリア（ソート設定は保持）
- has_active_filters: フィルターが適用されているか判定
- matches_filter: アイテムがフィルター条件にマッチするか判定

## Test Cases

### TC-001: add_language
- Given: デフォルトのFilterState
- When: add_language(state, "Rust")を実行
- Then: languagesに"Rust"が追加される

### TC-002: toggle_language
- Given: デフォルトのFilterState
- When: toggle_language(state, "Rust")を実行（追加）
- Then: languagesに"Rust"が含まれる
- When: toggle_language(state, "Rust")を再度実行（削除）
- Then: languagesに"Rust"が含まれない

### TC-003: has_active_filters
- Given: デフォルトのFilterState
- When: has_active_filters(&state)を実行
- Then: falseが返される（フィルターが適用されていない）
- When: add_language(state, "Rust")を実行後、has_active_filters(&state)を実行
- Then: trueが返される（フィルターが適用されている）

### TC-004: clear_all
- Given: 言語フィルターが設定されたFilterState
- When: clear_all(state)を実行
- Then: 言語フィルターがクリアされ、has_active_filtersがfalseを返す（ソート設定は保持される）

