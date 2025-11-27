# Task 6: Search & Filter Concepts 実装

## 1. 目的と背景

### なぜこのタスクが必要か
コントリビューター一覧やリポジトリ一覧で、ユーザーが目的のデータを素早く見つけられるように、検索・フィルター機能が必要です。Legible Architecture の原則に従い、Search と Filter を独立した Concept として実装します。

### 完成時のユーザー体験
- テキストボックスに入力すると、リアルタイムで結果がフィルタリングされる
- 複数の条件（言語、ステータス等）を組み合わせてフィルタリングできる
- フィルター状態がURLに反映され、共有可能

---

## 2. 前提条件

### 依存するタスク
- ✅ Task 5: GitHub API 実装

### 必要な知識
- Leptos のシグナル（signal）
- Legible Architecture の Concept パターン

---

## 3. 作成ファイル一覧

### Search Concept
| ファイル | 内容 |
|---------|------|
| `app/src/concepts/search/state.rs` | 検索状態の型定義 |
| `app/src/concepts/search/actions.rs` | 検索ロジック |
| `app/src/concepts/search/search.spec.md` | 仕様書 |
| `app/src/concepts/search/tests.rs` | テスト |
| `app/src/concepts/search/mod.rs` | モジュール定義 |

### Filter Concept
| ファイル | 内容 |
|---------|------|
| `app/src/concepts/filter/state.rs` | フィルター状態の型定義 |
| `app/src/concepts/filter/actions.rs` | フィルタリングロジック |
| `app/src/concepts/filter/filter.spec.md` | 仕様書 |
| `app/src/concepts/filter/tests.rs` | テスト |
| `app/src/concepts/filter/mod.rs` | モジュール定義 |

---

## 4. 実装手順

### Step 1: Search Concept の状態定義

`app/src/concepts/search/state.rs`:

```rust
use serde::{Deserialize, Serialize};

/// 検索状態
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SearchState {
    /// 検索クエリ
    pub query: String,
    /// 検索対象のフィールド
    pub search_fields: Vec<SearchField>,
    /// 検索結果の件数
    pub result_count: Option<usize>,
}

/// 検索対象フィールド
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SearchField {
    Name,
    Description,
    Author,
    Tag,
    All,
}

impl Default for SearchField {
    fn default() -> Self {
        SearchField::All
    }
}
```

### Step 2: Search Concept のアクション定義

`app/src/concepts/search/actions.rs`:

```rust
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
```

### Step 3: Filter Concept の状態定義

`app/src/concepts/filter/state.rs`:

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// フィルター状態
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FilterState {
    /// 選択された言語
    pub languages: HashSet<String>,
    /// 選択されたステータス
    pub statuses: HashSet<String>,
    /// 選択されたカテゴリ
    pub categories: HashSet<String>,
    /// ソート順
    pub sort_by: SortOption,
    /// ソート方向
    pub sort_direction: SortDirection,
}

/// ソートオプション
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SortOption {
    Name,
    UpdatedAt,
    Stars,
    Contributions,
    CreatedAt,
}

impl Default for SortOption {
    fn default() -> Self {
        SortOption::UpdatedAt
    }
}

/// ソート方向
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SortDirection {
    Ascending,
    Descending,
}

impl Default for SortDirection {
    fn default() -> Self {
        SortDirection::Descending
    }
}
```

### Step 4: Filter Concept のアクション定義

`app/src/concepts/filter/actions.rs`:

```rust
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
```

### Step 5: テストの作成

`app/src/concepts/search/tests.rs`:

```rust
#[cfg(test)]
mod tests {
    use crate::concepts::search::actions::*;
    use crate::concepts::search::state::*;

    #[test]
    fn test_update_query() {
        let state = SearchState::default();
        let new_state = update_query(state, "rust".to_string());
        assert_eq!(new_state.query, "rust");
    }

    #[test]
    fn test_clear_query() {
        let state = SearchState {
            query: "rust".to_string(),
            ..Default::default()
        };
        let new_state = clear_query(state);
        assert!(new_state.query.is_empty());
    }

    #[test]
    fn test_matches_query_case_insensitive() {
        assert!(matches_query("Hello World", "hello"));
        assert!(matches_query("Hello World", "WORLD"));
        assert!(!matches_query("Hello World", "foo"));
    }

    #[test]
    fn test_matches_empty_query() {
        assert!(matches_query("anything", ""));
    }
}
```

`app/src/concepts/filter/tests.rs`:

```rust
#[cfg(test)]
mod tests {
    use crate::concepts::filter::actions::*;
    use crate::concepts::filter::state::*;

    #[test]
    fn test_add_language() {
        let state = FilterState::default();
        let new_state = add_language(state, "Rust".to_string());
        assert!(new_state.languages.contains("Rust"));
    }

    #[test]
    fn test_toggle_language() {
        let state = FilterState::default();
        
        // 追加
        let state = toggle_language(state, "Rust".to_string());
        assert!(state.languages.contains("Rust"));
        
        // 削除
        let state = toggle_language(state, "Rust".to_string());
        assert!(!state.languages.contains("Rust"));
    }

    #[test]
    fn test_has_active_filters() {
        let state = FilterState::default();
        assert!(!has_active_filters(&state));

        let state = add_language(state, "Rust".to_string());
        assert!(has_active_filters(&state));
    }

    #[test]
    fn test_clear_all() {
        let state = add_language(FilterState::default(), "Rust".to_string());
        let state = clear_all(state);
        assert!(!has_active_filters(&state));
    }
}
```

### Step 6: mod.rs の作成

`app/src/concepts/search/mod.rs`:

```rust
/**
 * Search Concept
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this Concept):
 *   ├─ app/src/concepts/mod.rs
 *   ├─ app/src/pages/contributors/mod.rs
 *   └─ app/src/pages/repositories/mod.rs
 *
 * Related Documentation:
 *   └─ Spec: ./search.spec.md
 */

pub mod actions;
pub mod state;

#[cfg(test)]
mod tests;

pub use actions::*;
pub use state::*;
```

### Step 7: concepts/mod.rs の更新

`app/src/concepts/mod.rs` に追加：

```rust
pub mod search;
pub mod filter;
```

---

## 5. テスト実行

```bash
# Search Concept のテスト
cargo test -p app search::

# Filter Concept のテスト
cargo test -p app filter::

# 全テスト
bun run test:app
```

---

## 6. 完了条件チェックリスト

- [ ] Search Concept が実装されている
  - [ ] state.rs
  - [ ] actions.rs
  - [ ] tests.rs
  - [ ] mod.rs
  - [ ] search.spec.md
- [ ] Filter Concept が実装されている
  - [ ] state.rs
  - [ ] actions.rs
  - [ ] tests.rs
  - [ ] mod.rs
  - [ ] filter.spec.md
- [ ] concepts/mod.rs に両モジュールが追加されている
- [ ] 全テストが通る
- [ ] ビルドエラーがない

---

## 7. 参照ドキュメント

- Legible Architecture: `docs/00_prompts/legible-architecture.md`
- 既存の Concept 実装例: `app/src/concepts/user/`

---

## 8. 注意点

- **Concept の独立性**: Search と Filter は互いに依存しない。組み合わせは Synchronization で行う
- **純粋関数**: actions はすべて純粋関数として実装し、副作用を含まない
- **型安全性**: `HashSet` や `enum` を活用して型安全なフィルタリングを実現

