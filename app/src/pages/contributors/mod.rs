mod components;

use crate::components::auth_guard::AuthGuard;
use crate::components::container::Container;
use crate::concepts::search::{matches_query, SearchState, update_query};
use crate::concepts::user::{User, UserRole};
use components::{ContributorGrid, StatusFilter};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

/**
 * ContributorsPage Component
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
 *   ├─ app/src/concepts/user/mod.rs
 *   └─ ./components/mod.rs
 */
#[component]
pub fn ContributorsPage() -> impl IntoView {
    view! {
        <AuthGuard>
            <ContributorsContent />
        </AuthGuard>
    }
}

#[component]
fn ContributorsContent() -> impl IntoView {
    let navigate = use_navigate();
    let navigate_for_grid = navigate.clone();

    // モックデータ（Task 5完了後にリアルデータに置換）
    // TODO: GitHub APIから実際のデータを取得する
    use chrono::Utc;
    let mock_users = vec![
        User {
            username: "alice-dev".to_string(),
            display_name: "Alice Developer".to_string(),
            avatar_url: "https://api.dicebear.com/7.x/avataaars/svg?seed=Alice".to_string(),
            github_url: "https://github.com/alice-dev".to_string(),
            role: UserRole::CurrentEmployee,
            joined_at: Some(Utc::now()),
            left_at: None,
        },
        User {
            username: "bob-contrib".to_string(),
            display_name: "Bob Contributor".to_string(),
            avatar_url: "https://api.dicebear.com/7.x/avataaars/svg?seed=Bob".to_string(),
            github_url: "https://github.com/bob-contrib".to_string(),
            role: UserRole::CurrentEmployee,
            joined_at: Some(Utc::now()),
            left_at: None,
        },
        User {
            username: "charlie-alumni".to_string(),
            display_name: "Charlie Alumni".to_string(),
            avatar_url: "https://api.dicebear.com/7.x/avataaars/svg?seed=Charlie".to_string(),
            github_url: "https://github.com/charlie-alumni".to_string(),
            role: UserRole::Alumni,
            joined_at: Some(Utc::now()),
            left_at: Some(Utc::now()),
        },
        User {
            username: "dave-external".to_string(),
            display_name: "Dave External".to_string(),
            avatar_url: "https://api.dicebear.com/7.x/avataaars/svg?seed=Dave".to_string(),
            github_url: "https://github.com/dave-external".to_string(),
            role: UserRole::ExternalContributor,
            joined_at: None,
            left_at: None,
        },
    ];

    let (search_state, set_search_state) = signal(SearchState::default());
    let status_filter = RwSignal::new(None::<UserRole>);

    // フィルタリングされたユーザー
    let filtered_users = move || {
        let query = search_state.get().query;
        let status = status_filter.get();

        mock_users
            .iter()
            .filter(|user| {
                // 検索フィルター
                let matches_search = matches_query(&user.display_name, &query)
                    || matches_query(&user.username, &query);

                // ステータスフィルター
                let matches_status = status
                    .as_ref()
                    .map(|s| &user.role == s)
                    .unwrap_or(true);

                matches_search && matches_status
            })
            .cloned()
            .collect::<Vec<_>>()
    };

    // 検索ハンドラー
    let handle_search = move |query: String| {
        set_search_state.set(update_query(search_state.get(), query));
    };

    // ステータスフィルターハンドラー
    let status_filter_clone = status_filter.clone();
    let handle_status_change = move |status: Option<UserRole>| {
        status_filter_clone.set(status);
    };

    // カードクリックハンドラー
    let handle_card_click = move |username: String| {
        navigate(&format!("/portfolio/{}", username), Default::default());
    };

    view! {
        <Container>
            <div class="space-y-8">
                // ヘッダー
                <div>
                    <h1 class="text-4xl font-bold mb-2">"コントリビューター"</h1>
                    <p class="text-base-content/70">
                        "組織に貢献しているエンジニアの一覧"
                    </p>
                </div>

                // フィルター
                <div class="flex flex-col md:flex-row gap-4">
                    // 検索ボックス
                    <div class="form-control flex-1">
                        <input
                            type="text"
                            placeholder="名前またはユーザー名で検索..."
                            class="input input-bordered w-full"
                            prop:value=move || search_state.get().query
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                handle_search(value);
                            }
                        />
                    </div>

                    // ステータスフィルター
                    <StatusFilter
                        selected_status=status_filter
                        on_change=Callback::new(handle_status_change)
                    />
                </div>

                // 結果件数とグリッド
                {move || {
                    let users = filtered_users();
                    let navigate_for_callback = navigate_for_grid.clone();
                    let callback = Callback::new(move |username: String| {
                        navigate_for_callback(&format!("/portfolio/{}", username), Default::default());
                    });
                    view! {
                        <>
                            <div class="text-sm text-base-content/60">
                                {format!("{} 件のコントリビューター", users.len())}
                            </div>
                            <ContributorGrid
                                users=users
                                on_user_click=Some(callback)
                            />
                        </>
                    }
                }}
            </div>
        </Container>
    }
}

