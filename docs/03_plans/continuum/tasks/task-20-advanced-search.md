# Task 20: 高度な検索

## 1. 目的と背景

### なぜこのタスクが必要か
PRDの「5.2 知見共有機能」に記載されている「カテゴリや投稿者による高度な検索・フィルタリング機能」を実装します。Task 6で基本的なSearch/Filter Conceptを作成しましたが、このタスクではより高度な検索機能を実装します。

### 完成時のユーザー体験
- 複数の条件を組み合わせた検索ができる
- 検索結果がURLに反映され、共有可能
- 検索履歴が保存される
- インクリメンタルサーチ（入力中に結果が更新）

---

## 2. 前提条件

### 依存するタスク
- ✅ Task 6: Search & Filter Concepts
- ✅ Task 12: 知見共有一覧ページ

---

## 3. 作成ファイル一覧

| ファイル | 内容 |
|---------|------|
| `app/src/concepts/advanced_search/state.rs` | 高度な検索状態 |
| `app/src/concepts/advanced_search/actions.rs` | 高度な検索ロジック |
| `app/src/concepts/advanced_search/mod.rs` | モジュール定義 |
| `app/src/components/advanced_search/mod.rs` | 高度な検索UI |
| `app/src/components/advanced_search/search_bar.rs` | 検索バー |
| `app/src/components/advanced_search/filter_panel.rs` | フィルターパネル |
| `app/src/components/advanced_search/search_history.rs` | 検索履歴 |

---

## 4. 実装手順

### Step 1: Advanced Search Concept の状態定義

`app/src/concepts/advanced_search/state.rs`:

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// 高度な検索の状態
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdvancedSearchState {
    /// テキストクエリ
    pub query: String,
    /// フィルター条件
    pub filters: SearchFilters,
    /// ソート条件
    pub sort: SearchSort,
    /// 検索履歴
    pub history: Vec<SearchHistoryEntry>,
    /// 検索中フラグ
    pub searching: bool,
    /// 結果件数
    pub result_count: Option<usize>,
}

/// フィルター条件
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SearchFilters {
    /// 投稿者（複数選択可）
    pub authors: HashSet<String>,
    /// カテゴリ（複数選択可）
    pub categories: HashSet<String>,
    /// ラベル（複数選択可）
    pub labels: HashSet<String>,
    /// 期間（開始日）
    pub date_from: Option<String>,
    /// 期間（終了日）
    pub date_to: Option<String>,
    /// コメント数の最小値
    pub min_comments: Option<i32>,
    /// リアクション数の最小値
    pub min_reactions: Option<i32>,
}

impl SearchFilters {
    /// アクティブなフィルターの数を返す
    pub fn active_count(&self) -> usize {
        let mut count = 0;
        if !self.authors.is_empty() { count += 1; }
        if !self.categories.is_empty() { count += 1; }
        if !self.labels.is_empty() { count += 1; }
        if self.date_from.is_some() || self.date_to.is_some() { count += 1; }
        if self.min_comments.is_some() { count += 1; }
        if self.min_reactions.is_some() { count += 1; }
        count
    }

    /// フィルターが設定されているか
    pub fn has_filters(&self) -> bool {
        self.active_count() > 0
    }
}

/// ソート条件
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchSort {
    pub field: SortField,
    pub direction: SortDirection,
}

impl Default for SearchSort {
    fn default() -> Self {
        Self {
            field: SortField::Relevance,
            direction: SortDirection::Descending,
        }
    }
}

/// ソートフィールド
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SortField {
    Relevance,
    CreatedAt,
    UpdatedAt,
    Comments,
    Reactions,
}

impl SortField {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Relevance => "関連度",
            Self::CreatedAt => "作成日",
            Self::UpdatedAt => "更新日",
            Self::Comments => "コメント数",
            Self::Reactions => "リアクション数",
        }
    }
}

/// ソート方向
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SortDirection {
    Ascending,
    Descending,
}

/// 検索履歴エントリ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchHistoryEntry {
    pub query: String,
    pub filters: SearchFilters,
    pub searched_at: String,
    pub result_count: usize,
}
```

### Step 2: Advanced Search Concept のアクション定義

`app/src/concepts/advanced_search/actions.rs`:

```rust
use super::state::*;
use std::collections::HashSet;

/// クエリを更新
pub fn set_query(state: AdvancedSearchState, query: String) -> AdvancedSearchState {
    AdvancedSearchState { query, ..state }
}

/// フィルターを更新
pub fn set_filters(state: AdvancedSearchState, filters: SearchFilters) -> AdvancedSearchState {
    AdvancedSearchState { filters, ..state }
}

/// 投稿者フィルターをトグル
pub fn toggle_author(state: AdvancedSearchState, author: String) -> AdvancedSearchState {
    let mut filters = state.filters.clone();
    if filters.authors.contains(&author) {
        filters.authors.remove(&author);
    } else {
        filters.authors.insert(author);
    }
    AdvancedSearchState { filters, ..state }
}

/// カテゴリフィルターをトグル
pub fn toggle_category(state: AdvancedSearchState, category: String) -> AdvancedSearchState {
    let mut filters = state.filters.clone();
    if filters.categories.contains(&category) {
        filters.categories.remove(&category);
    } else {
        filters.categories.insert(category);
    }
    AdvancedSearchState { filters, ..state }
}

/// ラベルフィルターをトグル
pub fn toggle_label(state: AdvancedSearchState, label: String) -> AdvancedSearchState {
    let mut filters = state.filters.clone();
    if filters.labels.contains(&label) {
        filters.labels.remove(&label);
    } else {
        filters.labels.insert(label);
    }
    AdvancedSearchState { filters, ..state }
}

/// 期間フィルターを設定
pub fn set_date_range(
    state: AdvancedSearchState,
    from: Option<String>,
    to: Option<String>,
) -> AdvancedSearchState {
    let mut filters = state.filters.clone();
    filters.date_from = from;
    filters.date_to = to;
    AdvancedSearchState { filters, ..state }
}

/// ソートを設定
pub fn set_sort(state: AdvancedSearchState, sort: SearchSort) -> AdvancedSearchState {
    AdvancedSearchState { sort, ..state }
}

/// フィルターをクリア
pub fn clear_filters(state: AdvancedSearchState) -> AdvancedSearchState {
    AdvancedSearchState {
        filters: SearchFilters::default(),
        ..state
    }
}

/// すべてクリア
pub fn clear_all(state: AdvancedSearchState) -> AdvancedSearchState {
    AdvancedSearchState {
        query: String::new(),
        filters: SearchFilters::default(),
        result_count: None,
        ..state
    }
}

/// 検索履歴に追加
pub fn add_to_history(
    state: AdvancedSearchState,
    result_count: usize,
) -> AdvancedSearchState {
    let entry = SearchHistoryEntry {
        query: state.query.clone(),
        filters: state.filters.clone(),
        searched_at: chrono::Utc::now().to_rfc3339(),
        result_count,
    };

    let mut history = state.history.clone();
    // 重複を除去
    history.retain(|h| h.query != entry.query || h.filters != entry.filters);
    // 先頭に追加
    history.insert(0, entry);
    // 最大10件
    history.truncate(10);

    AdvancedSearchState { history, ..state }
}

/// 検索結果数を設定
pub fn set_result_count(state: AdvancedSearchState, count: usize) -> AdvancedSearchState {
    AdvancedSearchState {
        result_count: Some(count),
        searching: false,
        ..state
    }
}

/// 検索中フラグを設定
pub fn set_searching(state: AdvancedSearchState, searching: bool) -> AdvancedSearchState {
    AdvancedSearchState { searching, ..state }
}

/// 検索状態をURLクエリパラメータに変換
pub fn to_url_params(state: &AdvancedSearchState) -> String {
    let mut params = vec![];

    if !state.query.is_empty() {
        params.push(format!("q={}", urlencoding::encode(&state.query)));
    }

    if !state.filters.authors.is_empty() {
        let authors: Vec<_> = state.filters.authors.iter().collect();
        params.push(format!("author={}", authors.join(",")));
    }

    if !state.filters.categories.is_empty() {
        let categories: Vec<_> = state.filters.categories.iter().collect();
        params.push(format!("category={}", categories.join(",")));
    }

    if params.is_empty() {
        String::new()
    } else {
        format!("?{}", params.join("&"))
    }
}

/// URLクエリパラメータから検索状態を復元
pub fn from_url_params(params: &str) -> AdvancedSearchState {
    let mut state = AdvancedSearchState::default();

    for pair in params.trim_start_matches('?').split('&') {
        let mut parts = pair.splitn(2, '=');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            match key {
                "q" => {
                    state.query = urlencoding::decode(value)
                        .map(|s| s.to_string())
                        .unwrap_or_default();
                }
                "author" => {
                    state.filters.authors = value.split(',').map(String::from).collect();
                }
                "category" => {
                    state.filters.categories = value.split(',').map(String::from).collect();
                }
                _ => {}
            }
        }
    }

    state
}
```

### Step 3: SearchBar コンポーネント

`app/src/components/advanced_search/search_bar.rs`:

```rust
/**
 * SearchBar Component
 *
 * インクリメンタルサーチ対応の検索バー
 */

use leptos::prelude::*;

#[component]
pub fn SearchBar(
    value: String,
    on_change: Callback<String>,
    #[prop(optional)] placeholder: Option<&'static str>,
    #[prop(optional)] debounce_ms: Option<u32>,
) -> impl IntoView {
    let (internal_value, set_internal_value) = signal(value);
    let debounce = debounce_ms.unwrap_or(300);

    #[cfg(feature = "hydrate")]
    {
        use leptos::task::spawn_local;
        use gloo_timers::future::TimeoutFuture;
        use std::sync::atomic::{AtomicU32, Ordering};
        use std::sync::Arc;

        let counter = Arc::new(AtomicU32::new(0));

        Effect::new(move |_| {
            let current_value = internal_value.get();
            let counter = counter.clone();
            let on_change = on_change.clone();
            let id = counter.fetch_add(1, Ordering::SeqCst);

            spawn_local(async move {
                TimeoutFuture::new(debounce).await;
                // 最新のリクエストのみ実行
                if id == counter.load(Ordering::SeqCst) - 1 {
                    on_change.call(current_value);
                }
            });
        });
    }

    view! {
        <div class="form-control w-full">
            <div class="relative">
                <input
                    type="text"
                    class="input input-bordered w-full pl-10"
                    placeholder=placeholder.unwrap_or("検索...")
                    prop:value=move || internal_value.get()
                    on:input=move |ev| {
                        set_internal_value.set(event_target_value(&ev));
                    }
                />
                <svg
                    class="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-base-content/40"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                    />
                </svg>
            </div>
        </div>
    }
}
```

### Step 4: FilterPanel コンポーネント

`app/src/components/advanced_search/filter_panel.rs`:

```rust
/**
 * FilterPanel Component
 */

use crate::concepts::advanced_search::{SearchFilters, SearchSort, SortField};
use leptos::prelude::*;

#[component]
pub fn FilterPanel(
    filters: SearchFilters,
    sort: SearchSort,
    available_authors: Vec<String>,
    available_categories: Vec<String>,
    available_labels: Vec<String>,
    on_filters_change: Callback<SearchFilters>,
    on_sort_change: Callback<SearchSort>,
    on_clear: Callback<()>,
) -> impl IntoView {
    view! {
        <div class="card bg-base-200">
            <div class="card-body">
                <div class="flex items-center justify-between mb-4">
                    <h3 class="font-medium">"フィルター"</h3>
                    {(filters.has_filters()).then(|| {
                        view! {
                            <button
                                class="btn btn-ghost btn-xs"
                                on:click=move |_| on_clear.call(())
                            >
                                "クリア"
                            </button>
                        }
                    })}
                </div>

                // 投稿者フィルター
                <div class="form-control mb-4">
                    <label class="label">
                        <span class="label-text">"投稿者"</span>
                    </label>
                    <div class="flex flex-wrap gap-2">
                        {available_authors
                            .into_iter()
                            .map(|author| {
                                let is_selected = filters.authors.contains(&author);
                                let author_clone = author.clone();
                                let filters_clone = filters.clone();
                                let on_filters_change = on_filters_change.clone();

                                view! {
                                    <button
                                        class=move || if is_selected {
                                            "btn btn-primary btn-xs"
                                        } else {
                                            "btn btn-ghost btn-xs"
                                        }
                                        on:click=move |_| {
                                            let mut new_filters = filters_clone.clone();
                                            if new_filters.authors.contains(&author_clone) {
                                                new_filters.authors.remove(&author_clone);
                                            } else {
                                                new_filters.authors.insert(author_clone.clone());
                                            }
                                            on_filters_change.call(new_filters);
                                        }
                                    >
                                        {author.clone()}
                                    </button>
                                }
                            })
                            .collect_view()}
                    </div>
                </div>

                // カテゴリフィルター
                <div class="form-control mb-4">
                    <label class="label">
                        <span class="label-text">"カテゴリ"</span>
                    </label>
                    <div class="flex flex-wrap gap-2">
                        {available_categories
                            .into_iter()
                            .map(|category| {
                                let is_selected = filters.categories.contains(&category);
                                let category_clone = category.clone();
                                let filters_clone = filters.clone();
                                let on_filters_change = on_filters_change.clone();

                                view! {
                                    <button
                                        class=move || if is_selected {
                                            "btn btn-secondary btn-xs"
                                        } else {
                                            "btn btn-ghost btn-xs"
                                        }
                                        on:click=move |_| {
                                            let mut new_filters = filters_clone.clone();
                                            if new_filters.categories.contains(&category_clone) {
                                                new_filters.categories.remove(&category_clone);
                                            } else {
                                                new_filters.categories.insert(category_clone.clone());
                                            }
                                            on_filters_change.call(new_filters);
                                        }
                                    >
                                        {category.clone()}
                                    </button>
                                }
                            })
                            .collect_view()}
                    </div>
                </div>

                // ソート
                <div class="form-control">
                    <label class="label">
                        <span class="label-text">"並び順"</span>
                    </label>
                    <select
                        class="select select-bordered select-sm"
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            let field = match value.as_str() {
                                "created" => SortField::CreatedAt,
                                "updated" => SortField::UpdatedAt,
                                "comments" => SortField::Comments,
                                "reactions" => SortField::Reactions,
                                _ => SortField::Relevance,
                            };
                            on_sort_change.call(SearchSort {
                                field,
                                direction: sort.direction.clone(),
                            });
                        }
                    >
                        <option value="relevance">"関連度"</option>
                        <option value="created">"作成日"</option>
                        <option value="updated">"更新日"</option>
                        <option value="comments">"コメント数"</option>
                        <option value="reactions">"リアクション数"</option>
                    </select>
                </div>
            </div>
        </div>
    }
}
```

---

## 5. 完了条件チェックリスト

- [ ] AdvancedSearch Concept が実装されている
- [ ] SearchBar コンポーネントが実装されている
- [ ] FilterPanel コンポーネントが実装されている
- [ ] インクリメンタルサーチが動作する
- [ ] 検索状態がURLに反映される
- [ ] フィルターの組み合わせが動作する
- [ ] ビルドエラーがない

---

## 6. 参照ドキュメント

- PRD: `PRD.md` - セクション 5.2
- Task 6: `task-06-search-filter-concepts.md`

