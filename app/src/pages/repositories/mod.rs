mod components;

use crate::components::auth_guard::AuthGuard;
use crate::components::container::Container;
use crate::concepts::filter::{matches_language, set_sort, FilterState, SortDirection, SortOption};
use crate::concepts::repository::Repository;
use crate::concepts::search::{matches_query, update_query, SearchState};
use components::{LanguageFilter, RepositoryTable, SortControl};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use std::collections::HashSet;

/**
 * RepositoriesPage Component
 *
 * DEPENDENCY MAP:
 *
 * Parents:
 *   └─ app/src/lib.rs (routing)
 *
 * Dependencies:
 *   ├─ app/src/components/auth_guard/mod.rs
 *   ├─ app/src/components/container/mod.rs
 *   ├─ app/src/concepts/search/mod.rs
 *   ├─ app/src/concepts/filter/mod.rs
 *   ├─ app/src/concepts/repository/mod.rs
 *   └─ ./components/mod.rs
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

    // モックデータ（Task 5完了後にリアルデータに置換）
    // TODO: GitHub APIから実際のデータを取得する
    use chrono::Utc;
    let mock_repositories = vec![
        Repository {
            name: "continuum".to_string(),
            full_name: "org/continuum".to_string(),
            description: Some("エンジニアの成長を支援するプラットフォーム".to_string()),
            stars: 128,
            language: Some("Rust".to_string()),
            updated_at: Utc::now(),
            contributors: vec![],
        },
        Repository {
            name: "api-gateway".to_string(),
            full_name: "org/api-gateway".to_string(),
            description: Some("API Gateway service".to_string()),
            stars: 56,
            language: Some("Go".to_string()),
            updated_at: Utc::now(),
            contributors: vec![],
        },
        Repository {
            name: "frontend-components".to_string(),
            full_name: "org/frontend-components".to_string(),
            description: Some("Shared UI components".to_string()),
            stars: 89,
            language: Some("TypeScript".to_string()),
            updated_at: Utc::now(),
            contributors: vec![],
        },
    ];

    // 利用可能な言語を抽出
    let available_languages: Vec<String> = {
        let langs: HashSet<String> = mock_repositories
            .iter()
            .filter_map(|r| r.language.clone())
            .collect();
        let mut sorted: Vec<String> = langs.into_iter().collect();
        sorted.sort();
        sorted
    };

    let (search_state, set_search_state) = signal(SearchState::default());
    let (filter_state, set_filter_state) = signal(FilterState::default());

    // フィルタリング・ソート
    let filtered_repositories = move || {
        let query = search_state.get().query;
        let filter = filter_state.get();

        let mut filtered: Vec<_> = mock_repositories
            .iter()
            .filter(|r| {
                // 検索フィルター
                let matches_search = matches_query(&r.name, &query)
                    || r.description
                        .as_ref()
                        .map(|d| matches_query(d, &query))
                        .unwrap_or(false);

                // 言語フィルター
                let matches_lang = matches_language(r, &filter, |r| r.language.as_deref());

                matches_search && matches_lang
            })
            .cloned()
            .collect();

        // ソート
        filtered.sort_by(|a, b| {
            let cmp = match filter.sort_by {
                SortOption::Stars => a.stars.cmp(&b.stars),
                SortOption::Name => a.name.cmp(&b.name),
                SortOption::UpdatedAt => a.updated_at.cmp(&b.updated_at),
                _ => a.updated_at.cmp(&b.updated_at),
            };
            match filter.sort_direction {
                SortDirection::Ascending => cmp,
                SortDirection::Descending => cmp.reverse(),
            }
        });

        filtered
    };

    // 検索ハンドラー
    let handle_search = move |query: String| {
        set_search_state.set(update_query(search_state.get(), query));
    };

    // 言語フィルターハンドラー
    let handle_language_change = move |languages: Vec<String>| {
        let mut new_filter = filter_state.get();
        new_filter.languages = languages.into_iter().collect();
        set_filter_state.set(new_filter);
    };

    let selected_languages_signal = RwSignal::new(Vec::<String>::new());
    Effect::new(move |_| {
        let langs: Vec<String> = filter_state.get().languages.iter().cloned().collect();
        selected_languages_signal.set(langs);
    });

    view! {
        <Container>
            <div class="space-y-8">
                // ヘッダー
                <div>
                    <h1 class="text-4xl font-bold mb-2">"リポジトリ"</h1>
                    <p class="text-base-content/70">
                        "組織のパブリックリポジトリ一覧"
                    </p>
                </div>

                // フィルターとソート
                <div class="flex flex-col md:flex-row gap-4 justify-between">
                    // 検索ボックス
                    <div class="form-control flex-1 max-w-md">
                        <input
                            type="text"
                            placeholder="リポジトリ名で検索..."
                            class="input input-bordered w-full"
                            prop:value=move || search_state.get().query
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                handle_search(value);
                            }
                        />
                    </div>

                    // フィルターとソートコントロール
                    <div class="flex gap-2">
                        <LanguageFilter
                            selected_languages=selected_languages_signal
                            available_languages=available_languages.clone()
                            on_change=Callback::new(handle_language_change)
                        />
                        {move || {
                            let sort_by_value = filter_state.get().sort_by;
                            let direction_value = filter_state.get().sort_direction;
                            let handle_sort_change_clone = move |sort: SortOption, dir: SortDirection| {
                                set_filter_state.set(set_sort(filter_state.get(), sort, dir));
                            };
                            view! {
                                <SortControl
                                    sort_by=sort_by_value
                                    direction=direction_value
                                    on_change=Callback::new(move |(sort, dir)| handle_sort_change_clone(sort, dir))
                                />
                            }
                        }}
                    </div>
                </div>

                // 結果件数とテーブル
                {move || {
                    let repos = filtered_repositories();
                    let navigate_for_callback = navigate.clone();
                    let callback = Callback::new(move |full_name: String| {
                        navigate_for_callback(&format!("/repository/{}", full_name), Default::default());
                    });
                    view! {
                        <>
                            <div class="text-sm text-base-content/60">
                                {format!("{} 件のリポジトリ", repos.len())}
                            </div>
                            <RepositoryTable
                                repositories=repos
                                on_repo_click=callback
                            />
                        </>
                    }
                }}
            </div>
        </Container>
    }
}
