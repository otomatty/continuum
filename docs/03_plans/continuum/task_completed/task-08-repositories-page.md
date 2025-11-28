# Task 8: リポジトリ一覧ページ

## 1. 目的と背景

### なぜこのタスクが必要か
組織が管理するパブリックリポジトリの一覧を表示し、各リポジトリの基本情報（Star数、言語、最終更新日など）を確認できるページが必要です。

### 完成時のユーザー体験
- テーブル形式でリポジトリ一覧が表示される
- リポジトリ名で検索できる
- 言語でフィルタリングできる
- Star数、更新日などでソートできる
- 各リポジトリをクリックすると詳細ページに遷移する

---

## 2. 前提条件

### 依存するタスク
- ✅ Task 5: GitHub API 実装
- ✅ Task 6: Search & Filter Concepts
- ✅ Task 7: コントリビューター一覧ページ（参考実装）

---

## 3. 作成ファイル一覧

| ファイル | 内容 |
|---------|------|
| `app/src/pages/repositories/mod.rs` | ページコンポーネント |
| `app/src/pages/repositories/components/mod.rs` | コンポーネント再エクスポート |
| `app/src/pages/repositories/components/repository_table.rs` | リポジトリテーブル |
| `app/src/pages/repositories/components/language_filter.rs` | 言語フィルター |
| `app/src/pages/repositories/components/sort_control.rs` | ソートコントロール |

### 更新ファイル
| ファイル | 変更内容 |
|---------|---------|
| `app/src/pages/mod.rs` | `repositories` モジュール追加 |
| `app/src/lib.rs` | `/repositories` ルート追加 |

---

## 4. 実装手順

### Step 1: ディレクトリ構造の作成

```bash
mkdir -p app/src/pages/repositories/components
```

### Step 2: リポジトリデータ型の定義

`app/src/pages/repositories/components/repository_table.rs`:

```rust
/**
 * RepositoryTable Component
 *
 * DEPENDENCY MAP:
 *
 * Parents:
 *   └─ app/src/pages/repositories/mod.rs
 *
 * Dependencies:
 *   ├─ leptos::prelude
 *   └─ crate::components::table::*
 */

use crate::components::badge::Badge;
use leptos::prelude::*;

/// リポジトリ情報
#[derive(Debug, Clone, PartialEq)]
pub struct RepositoryItem {
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub stars: i32,
    pub forks: i32,
    pub language: Option<String>,
    pub language_color: Option<String>,
    pub updated_at: String,
    pub contributors_count: i32,
}

#[component]
pub fn RepositoryTable(
    repositories: Vec<RepositoryItem>,
    #[prop(optional)] on_row_click: Option<Callback<String>>,
) -> impl IntoView {
    view! {
        <div class="overflow-x-auto">
            <table class="table table-zebra">
                <thead>
                    <tr>
                        <th>"リポジトリ"</th>
                        <th>"言語"</th>
                        <th class="text-right">"Star"</th>
                        <th class="text-right">"Fork"</th>
                        <th class="text-right">"貢献者"</th>
                        <th>"最終更新"</th>
                    </tr>
                </thead>
                <tbody>
                    {repositories
                        .into_iter()
                        .map(|repo| {
                            let repo_name = repo.name.clone();
                            let callback = on_row_click.clone();

                            let handle_click = move |_| {
                                if let Some(cb) = &callback {
                                    cb.call(repo_name.clone());
                                }
                            };

                            view! {
                                <tr
                                    class="hover cursor-pointer"
                                    on:click=handle_click
                                >
                                    <td>
                                        <div class="flex flex-col">
                                            <span class="font-bold">{repo.name.clone()}</span>
                                            {repo.description.clone().map(|desc| {
                                                view! {
                                                    <span class="text-sm text-base-content/60 truncate max-w-xs">
                                                        {desc}
                                                    </span>
                                                }
                                            })}
                                        </div>
                                    </td>
                                    <td>
                                        {repo.language.clone().map(|lang| {
                                            let color = repo.language_color.clone().unwrap_or("#6e7681".to_string());
                                            view! {
                                                <div class="flex items-center gap-2">
                                                    <span
                                                        class="w-3 h-3 rounded-full"
                                                        style=format!("background-color: {}", color)
                                                    />
                                                    <span>{lang}</span>
                                                </div>
                                            }
                                        })}
                                    </td>
                                    <td class="text-right">
                                        <div class="flex items-center justify-end gap-1">
                                            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                                                <path d="M12 .587l3.668 7.568 8.332 1.151-6.064 5.828 1.48 8.279-7.416-3.967-7.417 3.967 1.481-8.279-6.064-5.828 8.332-1.151z"/>
                                            </svg>
                                            {repo.stars}
                                        </div>
                                    </td>
                                    <td class="text-right">{repo.forks}</td>
                                    <td class="text-right">{repo.contributors_count}</td>
                                    <td>
                                        <span class="text-sm text-base-content/60">
                                            {format_date(&repo.updated_at)}
                                        </span>
                                    </td>
                                </tr>
                            }
                        })
                        .collect_view()}
                </tbody>
            </table>
        </div>
    }
}

/// 日付をフォーマット
fn format_date(date_str: &str) -> String {
    // ISO 8601形式の日付を "YYYY/MM/DD" 形式に変換
    if let Some(date_part) = date_str.split('T').next() {
        date_part.replace('-', "/")
    } else {
        date_str.to_string()
    }
}
```

### Step 3: LanguageFilter コンポーネント

`app/src/pages/repositories/components/language_filter.rs`:

```rust
/**
 * LanguageFilter Component
 */

use leptos::prelude::*;
use std::collections::HashSet;

#[component]
pub fn LanguageFilter(
    languages: Vec<String>,
    selected: HashSet<String>,
    on_change: Callback<HashSet<String>>,
) -> impl IntoView {
    view! {
        <div class="dropdown dropdown-end">
            <div tabindex="0" role="button" class="btn btn-sm btn-ghost gap-2">
                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586a1 1 0 01-.293.707l-6.414 6.414a1 1 0 00-.293.707V17l-4 4v-6.586a1 1 0 00-.293-.707L3.293 7.293A1 1 0 013 6.586V4z" />
                </svg>
                "言語"
                {move || {
                    let count = selected.len();
                    if count > 0 {
                        Some(view! {
                            <span class="badge badge-sm badge-primary">{count}</span>
                        })
                    } else {
                        None
                    }
                }}
            </div>
            <ul tabindex="0" class="dropdown-content z-[1] menu p-2 shadow bg-base-200 rounded-box w-52 max-h-60 overflow-y-auto">
                {languages
                    .into_iter()
                    .map(|lang| {
                        let lang_for_check = lang.clone();
                        let lang_for_toggle = lang.clone();
                        let selected_clone = selected.clone();
                        let on_change = on_change.clone();

                        let is_selected = move || selected_clone.contains(&lang_for_check);

                        let handle_toggle = move |_| {
                            let mut new_selected = selected.clone();
                            if new_selected.contains(&lang_for_toggle) {
                                new_selected.remove(&lang_for_toggle);
                            } else {
                                new_selected.insert(lang_for_toggle.clone());
                            }
                            on_change.call(new_selected);
                        };

                        view! {
                            <li>
                                <label class="label cursor-pointer justify-start gap-2">
                                    <input
                                        type="checkbox"
                                        class="checkbox checkbox-sm"
                                        prop:checked=is_selected
                                        on:change=handle_toggle
                                    />
                                    <span>{lang.clone()}</span>
                                </label>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </div>
    }
}
```

### Step 4: SortControl コンポーネント

`app/src/pages/repositories/components/sort_control.rs`:

```rust
/**
 * SortControl Component
 */

use crate::concepts::filter::{SortDirection, SortOption};
use leptos::prelude::*;

#[component]
pub fn SortControl(
    sort_by: SortOption,
    direction: SortDirection,
    on_change: Callback<(SortOption, SortDirection)>,
) -> impl IntoView {
    let options = vec![
        (SortOption::UpdatedAt, "更新日"),
        (SortOption::Stars, "Star数"),
        (SortOption::Name, "名前"),
    ];

    let toggle_direction = move || {
        match direction {
            SortDirection::Ascending => SortDirection::Descending,
            SortDirection::Descending => SortDirection::Ascending,
        }
    };

    view! {
        <div class="flex items-center gap-2">
            <select
                class="select select-bordered select-sm"
                on:change=move |ev| {
                    let value = event_target_value(&ev);
                    let new_sort = match value.as_str() {
                        "updated" => SortOption::UpdatedAt,
                        "stars" => SortOption::Stars,
                        "name" => SortOption::Name,
                        _ => SortOption::UpdatedAt,
                    };
                    on_change.call((new_sort, direction.clone()));
                }
            >
                {options
                    .into_iter()
                    .map(|(opt, label)| {
                        let value = match opt {
                            SortOption::UpdatedAt => "updated",
                            SortOption::Stars => "stars",
                            SortOption::Name => "name",
                            _ => "updated",
                        };
                        let is_selected = std::mem::discriminant(&sort_by) == std::mem::discriminant(&opt);
                        view! {
                            <option value=value selected=is_selected>
                                {label}
                            </option>
                        }
                    })
                    .collect_view()}
            </select>

            <button
                class="btn btn-sm btn-ghost"
                on:click=move |_| {
                    on_change.call((sort_by.clone(), toggle_direction()));
                }
            >
                {move || {
                    match direction {
                        SortDirection::Descending => "↓",
                        SortDirection::Ascending => "↑",
                    }
                }}
            </button>
        </div>
    }
}
```

### Step 5: components/mod.rs

`app/src/pages/repositories/components/mod.rs`:

```rust
mod language_filter;
mod repository_table;
mod sort_control;

pub use language_filter::LanguageFilter;
pub use repository_table::{RepositoryItem, RepositoryTable};
pub use sort_control::SortControl;
```

### Step 6: ページコンポーネント

`app/src/pages/repositories/mod.rs`:

```rust
mod components;

use crate::components::auth_guard::AuthGuard;
use crate::components::container::Container;
use crate::concepts::filter::{SortDirection, SortOption};
use crate::concepts::search::{matches_query, SearchState, update_query};
use components::{LanguageFilter, RepositoryItem, RepositoryTable, SortControl};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use std::collections::HashSet;

/**
 * RepositoriesPage Component
 */
#[component]
pub fn RepositoriesPage() -> impl IntoView {
    view! {
        <AuthGuard>
            <RepositoriesContent />
        </AuthGuard>
    }
}

#[component]
fn RepositoriesContent() -> impl IntoView {
    let navigate = use_navigate();

    // モックデータ
    let mock_repositories = vec![
        RepositoryItem {
            name: "continuum".to_string(),
            description: Some("エンジニアの成長を支援するプラットフォーム".to_string()),
            url: "https://github.com/org/continuum".to_string(),
            stars: 128,
            forks: 24,
            language: Some("Rust".to_string()),
            language_color: Some("#dea584".to_string()),
            updated_at: "2025-11-25T10:00:00Z".to_string(),
            contributors_count: 12,
        },
        RepositoryItem {
            name: "api-gateway".to_string(),
            description: Some("API Gateway service".to_string()),
            url: "https://github.com/org/api-gateway".to_string(),
            stars: 56,
            forks: 8,
            language: Some("Go".to_string()),
            language_color: Some("#00ADD8".to_string()),
            updated_at: "2025-11-20T15:30:00Z".to_string(),
            contributors_count: 5,
        },
        RepositoryItem {
            name: "frontend-components".to_string(),
            description: Some("Shared UI components".to_string()),
            url: "https://github.com/org/frontend-components".to_string(),
            stars: 89,
            forks: 15,
            language: Some("TypeScript".to_string()),
            language_color: Some("#3178c6".to_string()),
            updated_at: "2025-11-22T09:00:00Z".to_string(),
            contributors_count: 8,
        },
    ];

    // 利用可能な言語を抽出
    let available_languages: Vec<String> = {
        let mut langs: HashSet<String> = mock_repositories
            .iter()
            .filter_map(|r| r.language.clone())
            .collect();
        let mut sorted: Vec<String> = langs.into_iter().collect();
        sorted.sort();
        sorted
    };

    let (search_state, set_search_state) = signal(SearchState::default());
    let (selected_languages, set_selected_languages) = signal(HashSet::<String>::new());
    let (sort_by, set_sort_by) = signal(SortOption::UpdatedAt);
    let (sort_direction, set_sort_direction) = signal(SortDirection::Descending);

    // フィルタリング・ソート
    let filtered_repositories = move || {
        let query = search_state.get().query;
        let langs = selected_languages.get();
        let sort = sort_by.get();
        let direction = sort_direction.get();

        let mut filtered: Vec<_> = mock_repositories
            .iter()
            .filter(|r| {
                let matches_search = matches_query(&r.name, &query)
                    || r.description
                        .as_ref()
                        .map(|d| matches_query(d, &query))
                        .unwrap_or(false);

                let matches_lang = langs.is_empty()
                    || r.language
                        .as_ref()
                        .map(|l| langs.contains(l))
                        .unwrap_or(false);

                matches_search && matches_lang
            })
            .cloned()
            .collect();

        // ソート
        filtered.sort_by(|a, b| {
            let cmp = match sort {
                SortOption::Stars => a.stars.cmp(&b.stars),
                SortOption::Name => a.name.cmp(&b.name),
                SortOption::UpdatedAt => a.updated_at.cmp(&b.updated_at),
                _ => a.updated_at.cmp(&b.updated_at),
            };
            match direction {
                SortDirection::Ascending => cmp,
                SortDirection::Descending => cmp.reverse(),
            }
        });

        filtered
    };

    let handle_search = move |query: String| {
        set_search_state.set(update_query(search_state.get(), query));
    };

    let handle_row_click = move |repo_name: String| {
        // TODO: オーナー名も含める必要がある
        navigate(&format!("/repository/org/{}", repo_name), Default::default());
    };

    view! {
        <Container>
            <div class="space-y-8">
                <div>
                    <h1 class="text-4xl font-bold mb-2">"リポジトリ"</h1>
                    <p class="text-base-content/70">"組織のパブリックリポジトリ一覧"</p>
                </div>

                <div class="flex flex-col md:flex-row gap-4 justify-between">
                    <div class="form-control flex-1 max-w-md">
                        <input
                            type="text"
                            placeholder="リポジトリ名で検索..."
                            class="input input-bordered w-full"
                            prop:value=move || search_state.get().query
                            on:input=move |ev| handle_search(event_target_value(&ev))
                        />
                    </div>

                    <div class="flex gap-2">
                        <LanguageFilter
                            languages=available_languages.clone()
                            selected=selected_languages.get()
                            on_change=Callback::new(move |langs| set_selected_languages.set(langs))
                        />
                        <SortControl
                            sort_by=sort_by.get()
                            direction=sort_direction.get()
                            on_change=Callback::new(move |(sort, dir)| {
                                set_sort_by.set(sort);
                                set_sort_direction.set(dir);
                            })
                        />
                    </div>
                </div>

                <div class="text-sm text-base-content/60">
                    {move || format!("{} 件のリポジトリ", filtered_repositories().len())}
                </div>

                <RepositoryTable
                    repositories=filtered_repositories()
                    on_row_click=Callback::new(handle_row_click)
                />
            </div>
        </Container>
    }
}
```

### Step 7: pages/mod.rs の更新

```rust
pub mod repositories;
pub use repositories::RepositoriesPage;
```

### Step 8: ルーティングの追加

`app/src/lib.rs`:

```rust
use pages::RepositoriesPage;

// Routes 内に追加
<Route path=StaticSegment("repositories") view=RepositoriesPage/>
```

---

## 5. 完了条件チェックリスト

- [ ] ディレクトリ構造が作成されている
- [ ] RepositoryTable コンポーネントが実装されている
- [ ] LanguageFilter コンポーネントが実装されている
- [ ] SortControl コンポーネントが実装されている
- [ ] ページコンポーネントが実装されている
- [ ] 検索機能が動作する
- [ ] 言語フィルターが動作する
- [ ] ソート機能が動作する
- [ ] ルーティングが設定されている
- [ ] ビルドエラーがない

---

## 6. 参照ドキュメント

- 画面設計: `docs/02_research/2025_11/20251121_screen-design-proposal.md` - セクション 2.1.G
- Filter Concept: `app/src/concepts/filter/`

