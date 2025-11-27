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
    FilterState { categories, ..state }
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
    !state.languages.is_empty() 
        || !state.statuses.is_empty() 
        || !state.categories.is_empty()
}

/// アイテムがフィルター条件にマッチするか
pub fn matches_filter<T, F>(item: &T, state: &FilterState, language_extractor: F) -> bool
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

