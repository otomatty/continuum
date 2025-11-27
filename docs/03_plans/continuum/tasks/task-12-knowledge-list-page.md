# Task 12: 知見共有一覧ページ

## 1. 目的と背景

### なぜこのタスクが必要か
PRDの「5.2 知見共有機能」に記載されている、GitHub Discussionsをデータソースとしたブログ形式のナレッジフィードを実装します。

### 完成時のユーザー体験
- カード形式で知見（Discussion）一覧が表示される
- カテゴリでフィルタリングできる
- タイトルや本文で検索できる
- 各カードをクリックすると詳細ページに遷移する
- 無限スクロールまたは「もっと見る」ボタンで追加読み込みできる

---

## 2. 前提条件

### 依存するタスク
- ✅ Task 10: Discussion Concepts
- ✅ Task 11: GitHub Discussions API 連携
- ✅ Task 6: Search & Filter Concepts

---

## 3. 作成ファイル一覧

| ファイル | 内容 |
|---------|------|
| `app/src/pages/knowledge/mod.rs` | ページコンポーネント |
| `app/src/pages/knowledge/components/mod.rs` | コンポーネント再エクスポート |
| `app/src/pages/knowledge/components/knowledge_card.rs` | 知見カード |
| `app/src/pages/knowledge/components/knowledge_grid.rs` | グリッドレイアウト |
| `app/src/pages/knowledge/components/category_sidebar.rs` | カテゴリサイドバー |
| `app/src/pages/knowledge/components/load_more_button.rs` | もっと見るボタン |

### 更新ファイル
| ファイル | 変更内容 |
|---------|---------|
| `app/src/pages/mod.rs` | `knowledge` モジュール追加 |
| `app/src/lib.rs` | `/knowledge` ルート追加 |

---

## 4. 実装手順

### Step 1: ディレクトリ構造の作成

```bash
mkdir -p app/src/pages/knowledge/components
```

### Step 2: KnowledgeCard コンポーネント

`app/src/pages/knowledge/components/knowledge_card.rs`:

```rust
/**
 * KnowledgeCard Component
 *
 * DEPENDENCY MAP:
 *
 * Parents:
 *   └─ app/src/pages/knowledge/components/knowledge_grid.rs
 *
 * Dependencies:
 *   ├─ leptos::prelude
 *   └─ crate::concepts::discussion::Discussion
 */

use crate::concepts::discussion::Discussion;
use leptos::prelude::*;

#[component]
pub fn KnowledgeCard(
    discussion: Discussion,
    #[prop(optional)] on_click: Option<Callback<String>>,
) -> impl IntoView {
    let id = discussion.id.clone();
    let id_for_click = id.clone();

    let handle_click = move |_| {
        if let Some(callback) = &on_click {
            callback.call(id_for_click.clone());
        }
    };

    // 日付のフォーマット
    let formatted_date = format_date(&discussion.created_at);

    // カテゴリの表示名
    let category_display = discussion.category.emoji
        .as_ref()
        .map(|e| format!("{} {}", e, discussion.category.name))
        .unwrap_or_else(|| discussion.category.name.clone());

    view! {
        <article
            class="card bg-base-200 hover:bg-base-300 transition-colors cursor-pointer"
            on:click=handle_click
        >
            <div class="card-body">
                // カテゴリタグ
                <div class="flex items-center gap-2 mb-2">
                    <span class="badge badge-outline badge-sm">
                        {category_display}
                    </span>
                    {(!discussion.labels.is_empty()).then(|| {
                        view! {
                            {discussion.labels.iter().take(3).map(|label| {
                                view! {
                                    <span class="badge badge-ghost badge-sm">{label.clone()}</span>
                                }
                            }).collect_view()}
                        }
                    })}
                </div>

                // タイトル
                <h2 class="card-title text-xl line-clamp-2">
                    {discussion.title.clone()}
                </h2>

                // プレビュー
                <p class="text-base-content/70 line-clamp-3 mt-2">
                    {discussion.body_preview.clone()}
                </p>

                // メタ情報
                <div class="flex items-center justify-between mt-4 pt-4 border-t border-base-300">
                    // 投稿者
                    <div class="flex items-center gap-2">
                        <div class="avatar">
                            <div class="w-8 h-8 rounded-full">
                                <img
                                    src=discussion.author.avatar_url.clone()
                                    alt=discussion.author.username.clone()
                                />
                            </div>
                        </div>
                        <span class="text-sm font-medium">
                            {discussion.author.username.clone()}
                        </span>
                    </div>

                    // 日付とリアクション
                    <div class="flex items-center gap-4 text-sm text-base-content/60">
                        <span>{formatted_date}</span>
                        <div class="flex items-center gap-1">
                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
                            </svg>
                            {discussion.comments_count}
                        </div>
                        <div class="flex items-center gap-1">
                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
                            </svg>
                            {discussion.reactions_count}
                        </div>
                    </div>
                </div>
            </div>
        </article>
    }
}

/// ISO 8601 日付を "YYYY年MM月DD日" 形式にフォーマット
fn format_date(date_str: &str) -> String {
    if let Some(date_part) = date_str.split('T').next() {
        let parts: Vec<&str> = date_part.split('-').collect();
        if parts.len() == 3 {
            return format!("{}年{}月{}日", parts[0], parts[1], parts[2]);
        }
    }
    date_str.to_string()
}
```

### Step 3: KnowledgeGrid コンポーネント

`app/src/pages/knowledge/components/knowledge_grid.rs`:

```rust
/**
 * KnowledgeGrid Component
 */

use super::knowledge_card::KnowledgeCard;
use crate::concepts::discussion::Discussion;
use leptos::prelude::*;

#[component]
pub fn KnowledgeGrid(
    discussions: Vec<Discussion>,
    #[prop(optional)] on_card_click: Option<Callback<String>>,
) -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {discussions
                .into_iter()
                .map(|discussion| {
                    let callback = on_card_click.clone();
                    view! {
                        <KnowledgeCard
                            discussion=discussion
                            on_click=callback
                        />
                    }
                })
                .collect_view()}
        </div>
    }
}
```

### Step 4: CategorySidebar コンポーネント

`app/src/pages/knowledge/components/category_sidebar.rs`:

```rust
/**
 * CategorySidebar Component
 */

use crate::concepts::category::Category;
use leptos::prelude::*;

#[component]
pub fn CategorySidebar(
    categories: Vec<Category>,
    selected_id: Option<String>,
    on_select: Callback<Option<String>>,
) -> impl IntoView {
    view! {
        <aside class="w-full lg:w-64 shrink-0">
            <div class="card bg-base-200">
                <div class="card-body">
                    <h3 class="card-title text-lg">"カテゴリ"</h3>
                    <ul class="menu p-0">
                        // すべてのカテゴリ
                        <li>
                            <a
                                class=move || {
                                    if selected_id.is_none() {
                                        "active"
                                    } else {
                                        ""
                                    }
                                }
                                on:click=move |_| on_select.call(None)
                            >
                                "📚 すべて"
                            </a>
                        </li>
                        // 各カテゴリ
                        {categories
                            .into_iter()
                            .map(|category| {
                                let cat_id = category.id.clone();
                                let cat_id_for_check = cat_id.clone();
                                let selected = selected_id.clone();
                                let on_select = on_select.clone();

                                let is_selected = move || {
                                    selected.as_ref().map(|s| s == &cat_id_for_check).unwrap_or(false)
                                };

                                view! {
                                    <li>
                                        <a
                                            class=move || {
                                                if is_selected() {
                                                    "active"
                                                } else {
                                                    ""
                                                }
                                            }
                                            on:click=move |_| on_select.call(Some(cat_id.clone()))
                                        >
                                            {category.display_name()}
                                            <span class="badge badge-sm">
                                                {category.discussions_count}
                                            </span>
                                        </a>
                                    </li>
                                }
                            })
                            .collect_view()}
                    </ul>
                </div>
            </div>
        </aside>
    }
}
```

### Step 5: LoadMoreButton コンポーネント

`app/src/pages/knowledge/components/load_more_button.rs`:

```rust
/**
 * LoadMoreButton Component
 */

use leptos::prelude::*;

#[component]
pub fn LoadMoreButton(
    loading: bool,
    has_more: bool,
    on_click: Callback<()>,
) -> impl IntoView {
    view! {
        <div class="flex justify-center py-8">
            {if loading {
                view! {
                    <span class="loading loading-spinner loading-lg" />
                }.into_any()
            } else if has_more {
                view! {
                    <button
                        class="btn btn-outline btn-wide"
                        on:click=move |_| on_click.call(())
                    >
                        "もっと見る"
                    </button>
                }.into_any()
            } else {
                view! {
                    <p class="text-base-content/60">"すべての知見を表示しました"</p>
                }.into_any()
            }}
        </div>
    }
}
```

### Step 6: components/mod.rs

`app/src/pages/knowledge/components/mod.rs`:

```rust
mod category_sidebar;
mod knowledge_card;
mod knowledge_grid;
mod load_more_button;

pub use category_sidebar::CategorySidebar;
pub use knowledge_card::KnowledgeCard;
pub use knowledge_grid::KnowledgeGrid;
pub use load_more_button::LoadMoreButton;
```

### Step 7: ページコンポーネント

`app/src/pages/knowledge/mod.rs`:

```rust
mod components;

use crate::components::auth_guard::AuthGuard;
use crate::components::container::Container;
use crate::concepts::category::Category;
use crate::concepts::discussion::{Discussion, DiscussionAuthor, DiscussionCategory};
use crate::concepts::search::{matches_query, SearchState, update_query};
use components::{CategorySidebar, KnowledgeGrid, LoadMoreButton};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

/**
 * KnowledgePage Component
 */
#[component]
pub fn KnowledgePage() -> impl IntoView {
    view! {
        <AuthGuard>
            <KnowledgeContent />
        </AuthGuard>
    }
}

#[component]
fn KnowledgeContent() -> impl IntoView {
    let navigate = use_navigate();

    // モックデータ（Task 11完了後にServer Functionに置換）
    let mock_categories = vec![
        Category {
            id: "cat-1".to_string(),
            name: "Tips & Tricks".to_string(),
            description: Some("開発に役立つ Tips".to_string()),
            emoji: Some("💡".to_string()),
            discussions_count: 15,
        },
        Category {
            id: "cat-2".to_string(),
            name: "Tech Blog".to_string(),
            description: Some("技術ブログ".to_string()),
            emoji: Some("📝".to_string()),
            discussions_count: 23,
        },
        Category {
            id: "cat-3".to_string(),
            name: "Q&A".to_string(),
            description: Some("質問と回答".to_string()),
            emoji: Some("❓".to_string()),
            discussions_count: 42,
        },
    ];

    let mock_discussions = vec![
        Discussion {
            id: "disc-1".to_string(),
            title: "Rust で非同期処理を効率的に書くコツ".to_string(),
            body: "# Rust での非同期処理\n\nRust の async/await は...".to_string(),
            body_preview: "Rust の async/await の使い方について解説します。効率的な非同期処理を書くためのコツを紹介します。".to_string(),
            author: DiscussionAuthor {
                username: "alice-dev".to_string(),
                display_name: Some("Alice Developer".to_string()),
                avatar_url: "https://api.dicebear.com/7.x/avataaars/svg?seed=Alice".to_string(),
            },
            category: DiscussionCategory {
                id: "cat-1".to_string(),
                name: "Tips & Tricks".to_string(),
                description: None,
                emoji: Some("💡".to_string()),
            },
            created_at: "2025-11-20T10:00:00Z".to_string(),
            updated_at: "2025-11-20T10:00:00Z".to_string(),
            comments_count: 8,
            reactions_count: 24,
            url: "https://github.com/org/repo/discussions/1".to_string(),
            labels: vec!["Rust".to_string(), "async".to_string()],
        },
        Discussion {
            id: "disc-2".to_string(),
            title: "Leptos で状態管理をシンプルにする方法".to_string(),
            body: "# Leptos の状態管理\n\nLeptos では signal を使って...".to_string(),
            body_preview: "Leptos フレームワークにおける状態管理のベストプラクティスを解説します。signal の使い方から、複雑な状態管理まで。".to_string(),
            author: DiscussionAuthor {
                username: "bob-dev".to_string(),
                display_name: Some("Bob Developer".to_string()),
                avatar_url: "https://api.dicebear.com/7.x/avataaars/svg?seed=Bob".to_string(),
            },
            category: DiscussionCategory {
                id: "cat-2".to_string(),
                name: "Tech Blog".to_string(),
                description: None,
                emoji: Some("📝".to_string()),
            },
            created_at: "2025-11-18T14:30:00Z".to_string(),
            updated_at: "2025-11-19T09:00:00Z".to_string(),
            comments_count: 12,
            reactions_count: 45,
            url: "https://github.com/org/repo/discussions/2".to_string(),
            labels: vec!["Leptos".to_string(), "Frontend".to_string()],
        },
    ];

    let (search_state, set_search_state) = signal(SearchState::default());
    let (selected_category, set_selected_category) = signal(None::<String>);
    let (loading, set_loading) = signal(false);
    let (has_more, _set_has_more) = signal(true);

    // フィルタリング
    let filtered_discussions = move || {
        let query = search_state.get().query;
        let category = selected_category.get();

        mock_discussions
            .iter()
            .filter(|d| {
                let matches_search = matches_query(&d.title, &query)
                    || matches_query(&d.body_preview, &query);

                let matches_category = category
                    .as_ref()
                    .map(|c| &d.category.id == c)
                    .unwrap_or(true);

                matches_search && matches_category
            })
            .cloned()
            .collect::<Vec<_>>()
    };

    let handle_search = move |query: String| {
        set_search_state.set(update_query(search_state.get(), query));
    };

    let handle_card_click = move |id: String| {
        navigate(&format!("/knowledge/{}", id), Default::default());
    };

    let handle_load_more = move |_| {
        set_loading.set(true);
        // TODO: Server Function を呼び出してページネーション
        // 一時的に3秒後にローディングを解除
        #[cfg(feature = "hydrate")]
        {
            use leptos::task::spawn_local;
            use gloo_timers::future::TimeoutFuture;

            spawn_local(async move {
                TimeoutFuture::new(1000).await;
                set_loading.set(false);
            });
        }
    };

    view! {
        <Container>
            <div class="space-y-8">
                // ヘッダー
                <div>
                    <h1 class="text-4xl font-bold mb-2">"知見共有"</h1>
                    <p class="text-base-content/70">
                        "組織内で共有されている知見やノウハウ"
                    </p>
                </div>

                // 検索
                <div class="form-control max-w-md">
                    <input
                        type="text"
                        placeholder="タイトルや内容で検索..."
                        class="input input-bordered w-full"
                        prop:value=move || search_state.get().query
                        on:input=move |ev| handle_search(event_target_value(&ev))
                    />
                </div>

                // メインコンテンツ
                <div class="flex flex-col lg:flex-row gap-8">
                    // サイドバー
                    <CategorySidebar
                        categories=mock_categories.clone()
                        selected_id=selected_category.get()
                        on_select=Callback::new(move |id| set_selected_category.set(id))
                    />

                    // グリッド
                    <div class="flex-1">
                        <div class="text-sm text-base-content/60 mb-4">
                            {move || format!("{} 件の知見", filtered_discussions().len())}
                        </div>

                        <KnowledgeGrid
                            discussions=filtered_discussions()
                            on_card_click=Callback::new(handle_card_click)
                        />

                        <LoadMoreButton
                            loading=loading.get()
                            has_more=has_more.get()
                            on_click=Callback::new(handle_load_more)
                        />
                    </div>
                </div>
            </div>
        </Container>
    }
}
```

### Step 8: ルーティングの追加

`app/src/lib.rs`:

```rust
use pages::KnowledgePage;

// Routes 内に追加
<Route path=StaticSegment("knowledge") view=KnowledgePage/>
```

---

## 5. 完了条件チェックリスト

- [ ] KnowledgeCard コンポーネントが実装されている
- [ ] KnowledgeGrid コンポーネントが実装されている
- [ ] CategorySidebar コンポーネントが実装されている
- [ ] LoadMoreButton コンポーネントが実装されている
- [ ] ページコンポーネントが実装されている
- [ ] 検索機能が動作する
- [ ] カテゴリフィルターが動作する
- [ ] ルーティングが設定されている
- [ ] ビルドエラーがない

---

## 6. 参照ドキュメント

- PRD: `PRD.md` - セクション 5.2
- 画面設計: `docs/02_research/2025_11/20251121_screen-design-proposal.md`
- Discussion Concept: `app/src/concepts/discussion/`

