/**
 * Category Concept - Actions
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   ├─ src/concepts/category/mod.rs
 *   └─ src/concepts/category/tests.rs
 *
 * Dependencies (External files that this file imports):
 *   └─ ./state.rs
 *
 * Related Documentation:
 *   └─ Spec: ./category.spec.md
 */
use super::state::*;

/// カテゴリ一覧を設定
pub fn set_categories(state: CategoryState, categories: Vec<Category>) -> CategoryState {
    CategoryState {
        categories,
        loading: false,
        error: None,
        ..state
    }
}

/// カテゴリを選択
pub fn select_category(state: CategoryState, category_id: Option<String>) -> CategoryState {
    CategoryState {
        selected_category_id: category_id,
        ..state
    }
}

/// 読み込み開始
pub fn set_loading(state: CategoryState, loading: bool) -> CategoryState {
    CategoryState { loading, ..state }
}

/// エラーを設定
pub fn set_error(state: CategoryState, error: String) -> CategoryState {
    CategoryState {
        error: Some(error),
        loading: false,
        ..state
    }
}

/// エラーをクリア
pub fn clear_error(state: CategoryState) -> CategoryState {
    CategoryState {
        error: None,
        ..state
    }
}

/// IDでカテゴリを検索
pub fn find_category_by_id<'a>(categories: &'a [Category], id: &str) -> Option<&'a Category> {
    categories.iter().find(|c| c.id == id)
}

/// 名前でカテゴリを検索
pub fn find_category_by_name<'a>(categories: &'a [Category], name: &str) -> Option<&'a Category> {
    categories.iter().find(|c| c.name == name)
}

/// Discussion数でソート（多い順）
pub fn sort_by_discussions_count_desc(mut categories: Vec<Category>) -> Vec<Category> {
    categories.sort_by(|a, b| b.discussions_count.cmp(&a.discussions_count));
    categories
}

/// 名前でソート（昇順）
pub fn sort_by_name(mut categories: Vec<Category>) -> Vec<Category> {
    categories.sort_by(|a, b| a.name.cmp(&b.name));
    categories
}

/// 選択中のカテゴリを取得
pub fn get_selected_category(state: &CategoryState) -> Option<&Category> {
    state
        .selected_category_id
        .as_ref()
        .and_then(|id| find_category_by_id(&state.categories, id))
}

/// カテゴリを追加
pub fn add_category(state: CategoryState, category: Category) -> CategoryState {
    let mut categories = state.categories;
    categories.push(category);
    CategoryState {
        categories,
        ..state
    }
}

/// カテゴリのDiscussion数を更新
pub fn update_discussions_count(
    state: CategoryState,
    category_id: &str,
    count: i32,
) -> CategoryState {
    let categories = state
        .categories
        .into_iter()
        .map(|c| {
            if c.id == category_id {
                Category {
                    discussions_count: count,
                    ..c
                }
            } else {
                c
            }
        })
        .collect();
    CategoryState {
        categories,
        ..state
    }
}
