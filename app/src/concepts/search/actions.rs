use super::state::{SearchField, SearchState};

/// 検索クエリを更新
pub fn update_query(state: SearchState, query: String) -> SearchState {
    SearchState { query, ..state }
}

/// 検索クエリをクリア
pub fn clear_query(state: SearchState) -> SearchState {
    SearchState {
        query: String::new(),
        result_count: None,
        ..state
    }
}

/// 検索対象フィールドを設定
pub fn set_search_fields(state: SearchState, fields: Vec<SearchField>) -> SearchState {
    SearchState {
        search_fields: fields,
        ..state
    }
}

/// 検索結果件数を更新
pub fn set_result_count(state: SearchState, count: usize) -> SearchState {
    SearchState {
        result_count: Some(count),
        ..state
    }
}

/// 文字列が検索クエリにマッチするか判定
pub fn matches_query(text: &str, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }
    text.to_lowercase().contains(&query.to_lowercase())
}

/// 複数フィールドのいずれかがマッチするか判定
pub fn matches_any_field<T, F>(item: &T, query: &str, field_extractors: &[F]) -> bool
where
    F: Fn(&T) -> &str,
{
    if query.is_empty() {
        return true;
    }
    field_extractors
        .iter()
        .any(|extract| matches_query(extract(item), query))
}

