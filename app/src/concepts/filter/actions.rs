use super::state::{FilterState, SortDirection, SortOption};
use std::collections::HashSet;

/// 言語フィルターを追加
pub fn add_language(state: FilterState, language: String) -> FilterState {
    let mut languages = state.languages.clone();
    languages.insert(language);
    FilterState { languages, ..state }
}

/// 言語フィルターを削除
pub fn remove_language(state: FilterState, language: &str) -> FilterState {
    let mut languages = state.languages.clone();
    languages.remove(language);
    FilterState { languages, ..state }
}

/// 言語フィルターをトグル
pub fn toggle_language(state: FilterState, language: String) -> FilterState {
    if state.languages.contains(&language) {
        remove_language(state, &language)
    } else {
        add_language(state, language)
    }
}

/// ステータスフィルターを設定
pub fn set_statuses(state: FilterState, statuses: HashSet<String>) -> FilterState {
    FilterState { statuses, ..state }
}

/// カテゴリフィルターを設定
pub fn set_categories(state: FilterState, categories: HashSet<String>) -> FilterState {
    FilterState {
        categories,
        ..state
    }
}

/// ソートオプションを設定
pub fn set_sort(state: FilterState, sort_by: SortOption, direction: SortDirection) -> FilterState {
    FilterState {
        sort_by,
        sort_direction: direction,
        ..state
    }
}

/// 全フィルターをクリア
pub fn clear_all(state: FilterState) -> FilterState {
    FilterState {
        sort_by: state.sort_by,
        sort_direction: state.sort_direction,
        ..FilterState::default()
    }
}

/// フィルターが適用されているか
pub fn has_active_filters(state: &FilterState) -> bool {
    !state.languages.is_empty() || !state.statuses.is_empty() || !state.categories.is_empty()
}

/// アイテムが言語フィルター条件にマッチするか
///
/// 言語フィルターのみを考慮します。
/// フィルターが空の場合は全てマッチとみなします。
pub fn matches_language<T, F>(item: &T, state: &FilterState, language_extractor: F) -> bool
where
    F: Fn(&T) -> Option<&str>,
{
    // 言語フィルターが空なら全てマッチ
    if state.languages.is_empty() {
        return true;
    }

    // アイテムの言語がフィルターに含まれているか
    match language_extractor(item) {
        Some(lang) => state.languages.contains(lang),
        None => false,
    }
}

/// アイテムがステータスフィルター条件にマッチするか
///
/// ステータスフィルターのみを考慮します。
/// フィルターが空の場合は全てマッチとみなします。
pub fn matches_status<T, F>(item: &T, state: &FilterState, status_extractor: F) -> bool
where
    F: Fn(&T) -> Option<&str>,
{
    // ステータスフィルターが空なら全てマッチ
    if state.statuses.is_empty() {
        return true;
    }

    // アイテムのステータスがフィルターに含まれているか
    match status_extractor(item) {
        Some(status) => state.statuses.contains(status),
        None => false,
    }
}

/// アイテムがカテゴリフィルター条件にマッチするか
///
/// カテゴリフィルターのみを考慮します。
/// フィルターが空の場合は全てマッチとみなします。
pub fn matches_category<T, F>(item: &T, state: &FilterState, category_extractor: F) -> bool
where
    F: Fn(&T) -> Option<&str>,
{
    // カテゴリフィルターが空なら全てマッチ
    if state.categories.is_empty() {
        return true;
    }

    // アイテムのカテゴリがフィルターに含まれているか
    match category_extractor(item) {
        Some(cat) => state.categories.contains(cat),
        None => false,
    }
}
