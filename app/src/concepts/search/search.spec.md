# Search Concept Specification

## Related Files

- Implementation: `src/concepts/search/mod.rs`
- State: `src/concepts/search/state.rs`
- Actions: `src/concepts/search/actions.rs`
- Tests: `src/concepts/search/tests.rs`

## Related Documentation

- Issue: (To be created if needed)
- Plan: `docs/03_plans/continuum/tasks/task-06-search-filter-concepts.md`
- Synchronizations: (To be created if needed)

## Requirements

### 責務
- 検索クエリの状態管理
- 検索対象フィールドの管理
- 検索結果件数の管理
- 文字列マッチングロジックの提供

### 状態構造
- SearchState: { query: String, search_fields: Vec<SearchField>, result_count: Option<usize> }
- SearchField: Name | Description | Author | Tag | All

### アクション
- update_query: 検索クエリを更新
- clear_query: 検索クエリをクリア
- set_search_fields: 検索対象フィールドを設定
- set_result_count: 検索結果件数を更新
- matches_query: 文字列が検索クエリにマッチするか判定（大文字小文字を区別しない）
- matches_any_field: 複数フィールドのいずれかがマッチするか判定

## Test Cases

### TC-001: update_query
- Given: デフォルトのSearchState
- When: update_query(state, "rust")を実行
- Then: queryが"rust"に更新される

### TC-002: clear_query
- Given: queryが"rust"のSearchState
- When: clear_query(state)を実行
- Then: queryが空文字列になり、result_countがNoneになる

### TC-003: matches_query_case_insensitive
- Given: テキスト"Hello World"とクエリ"hello"
- When: matches_query("Hello World", "hello")を実行
- Then: trueが返される（大文字小文字を区別しない）

### TC-004: matches_query_case_insensitive_uppercase
- Given: テキスト"Hello World"とクエリ"WORLD"
- When: matches_query("Hello World", "WORLD")を実行
- Then: trueが返される（大文字小文字を区別しない）

### TC-005: matches_query_no_match
- Given: テキスト"Hello World"とクエリ"foo"
- When: matches_query("Hello World", "foo")を実行
- Then: falseが返される

### TC-006: matches_empty_query
- Given: テキスト"anything"と空のクエリ""
- When: matches_query("anything", "")を実行
- Then: trueが返される（空クエリは全てにマッチ）

