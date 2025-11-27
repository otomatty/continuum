# Task 7: コントリビューター一覧ページ

## 1. 目的と背景

### なぜこのタスクが必要か
組織に貢献している全てのコントリビューターを一覧表示し、各人のプロフィールや貢献度を確認できるページが必要です。社内のエンジニア同士が互いの活動を知り、学び合うきっかけを提供します。

### 完成時のユーザー体験
- グリッドレイアウトでコントリビューターカードが表示される
- 名前やユーザー名で検索できる
- ステータス（現役/アルムナイ）でフィルタリングできる
- 各カードをクリックすると、そのユーザーのポートフォリオページに遷移する

---

## 2. 前提条件

### 依存するタスク
- ✅ Task 5: GitHub API 実装
- ✅ Task 6: Search & Filter Concepts

### 必要な知識
- Leptos コンポーネント
- Task 6 で実装した Search/Filter Concept の使い方

---

## 3. 作成ファイル一覧

| ファイル | 内容 |
|---------|------|
| `app/src/pages/contributors/mod.rs` | ページコンポーネント |
| `app/src/pages/contributors/components/mod.rs` | コンポーネント再エクスポート |
| `app/src/pages/contributors/components/contributor_card.rs` | コントリビューターカード |
| `app/src/pages/contributors/components/contributor_grid.rs` | グリッドレイアウト |
| `app/src/pages/contributors/components/status_filter.rs` | ステータスフィルター |

### 更新ファイル
| ファイル | 変更内容 |
|---------|---------|
| `app/src/pages/mod.rs` | `contributors` モジュール追加 |
| `app/src/lib.rs` | `/contributors` ルート追加 |

---

## 4. 実装手順

### Step 1: ディレクトリ構造の作成

```bash
mkdir -p app/src/pages/contributors/components
```

### Step 2: ContributorCard コンポーネント

`app/src/pages/contributors/components/contributor_card.rs`:

```rust
/**
 * ContributorCard Component
 *
 * DEPENDENCY MAP:
 *
 * Parents:
 *   └─ app/src/pages/contributors/components/contributor_grid.rs
 *
 * Dependencies:
 *   ├─ leptos::prelude
 *   └─ crate::components::avatar::Avatar
 */

use crate::components::avatar::Avatar;
use leptos::prelude::*;

/// コントリビューター情報
#[derive(Debug, Clone, PartialEq)]
pub struct Contributor {
    pub username: String,
    pub display_name: String,
    pub avatar_url: String,
    pub total_contributions: i32,
    pub status: ContributorStatus,
}

/// コントリビューターのステータス
#[derive(Debug, Clone, PartialEq)]
pub enum ContributorStatus {
    Active,    // 現役社員
    Alumni,    // 元社員
    External,  // 外部コントリビューター
}

impl ContributorStatus {
    pub fn label(&self) -> &'static str {
        match self {
            ContributorStatus::Active => "現役",
            ContributorStatus::Alumni => "アルムナイ",
            ContributorStatus::External => "外部",
        }
    }

    pub fn badge_class(&self) -> &'static str {
        match self {
            ContributorStatus::Active => "badge-success",
            ContributorStatus::Alumni => "badge-info",
            ContributorStatus::External => "badge-warning",
        }
    }
}

#[component]
pub fn ContributorCard(
    contributor: Contributor,
    #[prop(optional)] on_click: Option<Callback<String>>,
) -> impl IntoView {
    let username = contributor.username.clone();
    let username_for_click = username.clone();

    let handle_click = move |_| {
        if let Some(callback) = &on_click {
            callback.call(username_for_click.clone());
        }
    };

    view! {
        <div
            class="card bg-base-200 hover:bg-base-300 cursor-pointer transition-colors"
            on:click=handle_click
        >
            <div class="card-body items-center text-center p-6">
                // Avatar
                <div class="avatar mb-4">
                    <div class="w-20 rounded-full ring ring-primary ring-offset-base-100 ring-offset-2">
                        <img src=contributor.avatar_url.clone() alt=contributor.display_name.clone() />
                    </div>
                </div>

                // Name
                <h3 class="card-title text-lg">
                    {contributor.display_name.clone()}
                </h3>

                // Username
                <p class="text-sm text-base-content/60">
                    "@" {contributor.username.clone()}
                </p>

                // Status Badge
                <div class=format!("badge {} mt-2", contributor.status.badge_class())>
                    {contributor.status.label()}
                </div>

                // Contributions
                <div class="stat-value text-2xl mt-4">
                    {contributor.total_contributions}
                </div>
                <div class="stat-desc">
                    "コントリビューション"
                </div>
            </div>
        </div>
    }
}
```

### Step 3: ContributorGrid コンポーネント

`app/src/pages/contributors/components/contributor_grid.rs`:

```rust
/**
 * ContributorGrid Component
 *
 * DEPENDENCY MAP:
 *
 * Parents:
 *   └─ app/src/pages/contributors/mod.rs
 *
 * Dependencies:
 *   └─ super::contributor_card::{Contributor, ContributorCard}
 */

use super::contributor_card::{Contributor, ContributorCard};
use leptos::prelude::*;

#[component]
pub fn ContributorGrid(
    contributors: Vec<Contributor>,
    #[prop(optional)] on_card_click: Option<Callback<String>>,
) -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
            {contributors
                .into_iter()
                .map(|contributor| {
                    let callback = on_card_click.clone();
                    view! {
                        <ContributorCard
                            contributor=contributor
                            on_click=callback
                        />
                    }
                })
                .collect_view()}
        </div>
    }
}
```

### Step 4: StatusFilter コンポーネント

`app/src/pages/contributors/components/status_filter.rs`:

```rust
/**
 * StatusFilter Component
 *
 * DEPENDENCY MAP:
 *
 * Parents:
 *   └─ app/src/pages/contributors/mod.rs
 *
 * Dependencies:
 *   └─ super::contributor_card::ContributorStatus
 */

use super::contributor_card::ContributorStatus;
use leptos::prelude::*;

#[component]
pub fn StatusFilter(
    selected: Option<ContributorStatus>,
    on_change: Callback<Option<ContributorStatus>>,
) -> impl IntoView {
    let options = vec![
        (None, "すべて"),
        (Some(ContributorStatus::Active), "現役"),
        (Some(ContributorStatus::Alumni), "アルムナイ"),
        (Some(ContributorStatus::External), "外部"),
    ];

    view! {
        <div class="flex gap-2">
            {options
                .into_iter()
                .map(|(value, label)| {
                    let is_selected = selected == value;
                    let value_for_click = value.clone();
                    let on_change = on_change.clone();

                    view! {
                        <button
                            class=move || {
                                if is_selected {
                                    "btn btn-primary btn-sm"
                                } else {
                                    "btn btn-ghost btn-sm"
                                }
                            }
                            on:click=move |_| on_change.call(value_for_click.clone())
                        >
                            {label}
                        </button>
                    }
                })
                .collect_view()}
        </div>
    }
}
```

### Step 5: components/mod.rs

`app/src/pages/contributors/components/mod.rs`:

```rust
mod contributor_card;
mod contributor_grid;
mod status_filter;

pub use contributor_card::{Contributor, ContributorCard, ContributorStatus};
pub use contributor_grid::ContributorGrid;
pub use status_filter::StatusFilter;
```

### Step 6: ページコンポーネント

`app/src/pages/contributors/mod.rs`:

```rust
mod components;

use crate::components::auth_guard::AuthGuard;
use crate::components::container::Container;
use crate::concepts::search::{matches_query, SearchState, update_query};
use components::{Contributor, ContributorGrid, ContributorStatus, StatusFilter};
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

    // モックデータ（Task 5完了後にリアルデータに置換）
    let mock_contributors = vec![
        Contributor {
            username: "alice-dev".to_string(),
            display_name: "Alice Developer".to_string(),
            avatar_url: "https://api.dicebear.com/7.x/avataaars/svg?seed=Alice".to_string(),
            total_contributions: 156,
            status: ContributorStatus::Active,
        },
        Contributor {
            username: "bob-contrib".to_string(),
            display_name: "Bob Contributor".to_string(),
            avatar_url: "https://api.dicebear.com/7.x/avataaars/svg?seed=Bob".to_string(),
            total_contributions: 89,
            status: ContributorStatus::Active,
        },
        Contributor {
            username: "charlie-alumni".to_string(),
            display_name: "Charlie Alumni".to_string(),
            avatar_url: "https://api.dicebear.com/7.x/avataaars/svg?seed=Charlie".to_string(),
            total_contributions: 234,
            status: ContributorStatus::Alumni,
        },
    ];

    let (search_state, set_search_state) = signal(SearchState::default());
    let (status_filter, set_status_filter) = signal(None::<ContributorStatus>);

    // フィルタリングされたコントリビューター
    let filtered_contributors = move || {
        let query = search_state.get().query;
        let status = status_filter.get();

        mock_contributors
            .iter()
            .filter(|c| {
                // 検索フィルター
                let matches_search = matches_query(&c.display_name, &query)
                    || matches_query(&c.username, &query);

                // ステータスフィルター
                let matches_status = status
                    .as_ref()
                    .map(|s| &c.status == s)
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
    let handle_status_change = move |status: Option<ContributorStatus>| {
        set_status_filter.set(status);
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
                        selected=status_filter.get()
                        on_change=Callback::new(handle_status_change)
                    />
                </div>

                // 結果件数
                <div class="text-sm text-base-content/60">
                    {move || format!("{} 件のコントリビューター", filtered_contributors().len())}
                </div>

                // グリッド
                <ContributorGrid
                    contributors=filtered_contributors()
                    on_card_click=Callback::new(handle_card_click)
                />
            </div>
        </Container>
    }
}
```

### Step 7: pages/mod.rs の更新

`app/src/pages/mod.rs` に追加：

```rust
pub mod contributors;
pub use contributors::ContributorsPage;
```

### Step 8: ルーティングの追加

`app/src/lib.rs` のルート定義に追加：

```rust
// インポート追加
use pages::ContributorsPage;

// Routes 内に追加
<Route path=StaticSegment("contributors") view=ContributorsPage/>
```

---

## 5. テスト項目

### 手動テスト
- [ ] `/contributors` にアクセスしてページが表示される
- [ ] 検索ボックスに入力すると結果がフィルタリングされる
- [ ] ステータスボタンをクリックするとフィルタリングされる
- [ ] カードをクリックするとポートフォリオページに遷移する
- [ ] レスポンシブデザインが機能する

---

## 6. 完了条件チェックリスト

- [ ] `contributors/` ディレクトリが作成されている
- [ ] ContributorCard コンポーネントが実装されている
- [ ] ContributorGrid コンポーネントが実装されている
- [ ] StatusFilter コンポーネントが実装されている
- [ ] ページコンポーネントが実装されている
- [ ] 検索機能が動作する
- [ ] フィルター機能が動作する
- [ ] ルーティングが設定されている
- [ ] ビルドエラーがない

---

## 7. 参照ドキュメント

- 画面設計: `docs/02_research/2025_11/20251121_screen-design-proposal.md` - セクション 2.1.F
- Search Concept: `app/src/concepts/search/`
- 認証ガード: `app/src/components/auth_guard/`

---

## 8. 注意点

- **モックデータ**: 現時点ではモックデータを使用。Task 5完了後にリアルデータに置換
- **認証ガード**: このページは認証必須。`AuthGuard` でラップ済み
- **パフォーマンス**: 大量のコントリビューターがいる場合は仮想スクロールを検討

