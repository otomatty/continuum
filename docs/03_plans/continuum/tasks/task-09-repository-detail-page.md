# Task 9: リポジトリ詳細ページ

## 1. 目的と背景

### なぜこのタスクが必要か
各リポジトリの詳細情報（コントリビューター構成、言語構成、アクティビティなど）を表示するページが必要です。PRDに記載されている「Star数、最終更新日、コントリビューター構成比など」を可視化します。

### 完成時のユーザー体験
- リポジトリのヘッダー情報（名前、説明、Star/Fork数）が表示される
- コントリビューター構成が円グラフで表示される
- トップコントリビューターの一覧が表示される
- 言語構成がバーチャートで表示される

---

## 2. 前提条件

### 依存するタスク
- ✅ Task 5: GitHub API 実装
- ✅ Task 8: リポジトリ一覧ページ

---

## 3. 作成ファイル一覧

| ファイル | 内容 |
|---------|------|
| `app/src/pages/repository/mod.rs` | ページコンポーネント |
| `app/src/pages/repository/components/mod.rs` | コンポーネント再エクスポート |
| `app/src/pages/repository/components/repository_header.rs` | リポジトリヘッダー |
| `app/src/pages/repository/components/contributor_pie_chart.rs` | コントリビューター構成円グラフ |
| `app/src/pages/repository/components/top_contributors_list.rs` | トップコントリビューター一覧 |
| `app/src/pages/repository/components/language_bar_chart.rs` | 言語構成バーチャート |

### 更新ファイル
| ファイル | 変更内容 |
|---------|---------|
| `app/src/pages/mod.rs` | `repository` モジュール追加 |
| `app/src/lib.rs` | `/repository/:owner/:repo` ルート追加 |

---

## 4. 実装手順

### Step 1: ディレクトリ構造の作成

```bash
mkdir -p app/src/pages/repository/components
```

### Step 2: RepositoryHeader コンポーネント

`app/src/pages/repository/components/repository_header.rs`:

```rust
/**
 * RepositoryHeader Component
 */

use leptos::prelude::*;

#[derive(Debug, Clone)]
pub struct RepositoryDetail {
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub stars: i32,
    pub forks: i32,
    pub watchers: i32,
    pub language: Option<String>,
    pub language_color: Option<String>,
    pub updated_at: String,
    pub created_at: String,
    pub license: Option<String>,
}

#[component]
pub fn RepositoryHeader(repo: RepositoryDetail) -> impl IntoView {
    view! {
        <div class="card bg-base-200">
            <div class="card-body">
                // Title and GitHub link
                <div class="flex items-start justify-between">
                    <div>
                        <h1 class="text-3xl font-bold flex items-center gap-3">
                            <svg class="w-8 h-8" viewBox="0 0 24 24" fill="currentColor">
                                <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                            </svg>
                            {repo.name.clone()}
                        </h1>
                        {repo.description.clone().map(|desc| {
                            view! {
                                <p class="text-base-content/70 mt-2">{desc}</p>
                            }
                        })}
                    </div>
                    <a
                        href=repo.url.clone()
                        target="_blank"
                        rel="noopener noreferrer"
                        class="btn btn-outline btn-sm"
                    >
                        "GitHubで見る"
                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
                        </svg>
                    </a>
                </div>

                // Stats
                <div class="flex flex-wrap gap-6 mt-6">
                    <div class="flex items-center gap-2">
                        <svg class="w-5 h-5 text-warning" fill="currentColor" viewBox="0 0 24 24">
                            <path d="M12 .587l3.668 7.568 8.332 1.151-6.064 5.828 1.48 8.279-7.416-3.967-7.417 3.967 1.481-8.279-6.064-5.828 8.332-1.151z"/>
                        </svg>
                        <span class="font-semibold">{repo.stars}</span>
                        <span class="text-base-content/60">"Stars"</span>
                    </div>
                    <div class="flex items-center gap-2">
                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
                        </svg>
                        <span class="font-semibold">{repo.forks}</span>
                        <span class="text-base-content/60">"Forks"</span>
                    </div>
                    <div class="flex items-center gap-2">
                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                        </svg>
                        <span class="font-semibold">{repo.watchers}</span>
                        <span class="text-base-content/60">"Watchers"</span>
                    </div>
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
                    {repo.license.clone().map(|license| {
                        view! {
                            <div class="flex items-center gap-2">
                                <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
                                </svg>
                                <span>{license}</span>
                            </div>
                        }
                    })}
                </div>
            </div>
        </div>
    }
}
```

### Step 3: ContributorPieChart コンポーネント

`app/src/pages/repository/components/contributor_pie_chart.rs`:

```rust
/**
 * ContributorPieChart Component
 * 
 * CSSのみで円グラフを描画するシンプルな実装
 */

use leptos::prelude::*;

#[derive(Debug, Clone)]
pub struct ContributorContribution {
    pub username: String,
    pub avatar_url: String,
    pub contribution_count: i32,
    pub percentage: f32,
}

#[component]
pub fn ContributorPieChart(contributors: Vec<ContributorContribution>) -> impl IntoView {
    // 上位5名のみ表示、残りは「その他」にまとめる
    let (top_contributors, others): (Vec<_>, Vec<_>) = contributors
        .into_iter()
        .enumerate()
        .partition(|(i, _)| *i < 5);

    let others_percentage: f32 = others.iter().map(|(_, c)| c.percentage).sum();

    // CSS conic-gradient用の色
    let colors = vec![
        "#3b82f6", // blue
        "#10b981", // green
        "#f59e0b", // amber
        "#ef4444", // red
        "#8b5cf6", // violet
        "#6b7280", // gray (others)
    ];

    // conic-gradient を構築
    let mut gradient_parts: Vec<String> = vec![];
    let mut current_angle = 0.0f32;

    for (i, (_, contributor)) in top_contributors.iter().enumerate() {
        let start = current_angle;
        let end = current_angle + contributor.percentage * 3.6; // 100% = 360度
        gradient_parts.push(format!(
            "{} {}deg {}deg",
            colors[i], start, end
        ));
        current_angle = end;
    }

    if others_percentage > 0.0 {
        let start = current_angle;
        let end = 360.0;
        gradient_parts.push(format!(
            "{} {}deg {}deg",
            colors[5], start, end
        ));
    }

    let gradient_style = format!(
        "background: conic-gradient({}); width: 200px; height: 200px; border-radius: 50%;",
        gradient_parts.join(", ")
    );

    view! {
        <div class="card bg-base-200">
            <div class="card-body">
                <h2 class="card-title">"コントリビューター構成"</h2>
                <div class="flex flex-col md:flex-row items-center gap-8">
                    // 円グラフ
                    <div style=gradient_style />

                    // 凡例
                    <div class="flex flex-col gap-2">
                        {top_contributors
                            .into_iter()
                            .enumerate()
                            .map(|(i, (_, contributor))| {
                                view! {
                                    <div class="flex items-center gap-2">
                                        <div
                                            class="w-4 h-4 rounded"
                                            style=format!("background-color: {}", colors[i])
                                        />
                                        <span class="font-medium">{contributor.username.clone()}</span>
                                        <span class="text-base-content/60">
                                            {format!("{:.1}%", contributor.percentage)}
                                        </span>
                                    </div>
                                }
                            })
                            .collect_view()}
                        {(others_percentage > 0.0).then(|| {
                            view! {
                                <div class="flex items-center gap-2">
                                    <div
                                        class="w-4 h-4 rounded"
                                        style=format!("background-color: {}", colors[5])
                                    />
                                    <span class="font-medium">"その他"</span>
                                    <span class="text-base-content/60">
                                        {format!("{:.1}%", others_percentage)}
                                    </span>
                                </div>
                            }
                        })}
                    </div>
                </div>
            </div>
        </div>
    }
}
```

### Step 4: TopContributorsList コンポーネント

`app/src/pages/repository/components/top_contributors_list.rs`:

```rust
/**
 * TopContributorsList Component
 */

use super::contributor_pie_chart::ContributorContribution;
use leptos::prelude::*;

#[component]
pub fn TopContributorsList(
    contributors: Vec<ContributorContribution>,
    #[prop(optional)] on_click: Option<Callback<String>>,
) -> impl IntoView {
    view! {
        <div class="card bg-base-200">
            <div class="card-body">
                <h2 class="card-title">"トップコントリビューター"</h2>
                <div class="overflow-x-auto">
                    <table class="table">
                        <thead>
                            <tr>
                                <th>"#"</th>
                                <th>"コントリビューター"</th>
                                <th class="text-right">"コントリビューション"</th>
                                <th class="text-right">"割合"</th>
                            </tr>
                        </thead>
                        <tbody>
                            {contributors
                                .into_iter()
                                .enumerate()
                                .map(|(i, contributor)| {
                                    let username = contributor.username.clone();
                                    let callback = on_click.clone();

                                    let handle_click = move |_| {
                                        if let Some(cb) = &callback {
                                            cb.call(username.clone());
                                        }
                                    };

                                    view! {
                                        <tr
                                            class="hover cursor-pointer"
                                            on:click=handle_click
                                        >
                                            <td>{i + 1}</td>
                                            <td>
                                                <div class="flex items-center gap-3">
                                                    <div class="avatar">
                                                        <div class="mask mask-circle w-10 h-10">
                                                            <img
                                                                src=contributor.avatar_url.clone()
                                                                alt=contributor.username.clone()
                                                            />
                                                        </div>
                                                    </div>
                                                    <span class="font-bold">
                                                        {contributor.username.clone()}
                                                    </span>
                                                </div>
                                            </td>
                                            <td class="text-right">
                                                {contributor.contribution_count}
                                            </td>
                                            <td class="text-right">
                                                {format!("{:.1}%", contributor.percentage)}
                                            </td>
                                        </tr>
                                    }
                                })
                                .collect_view()}
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    }
}
```

### Step 5: LanguageBarChart コンポーネント

`app/src/pages/repository/components/language_bar_chart.rs`:

```rust
/**
 * LanguageBarChart Component
 */

use leptos::prelude::*;

#[derive(Debug, Clone)]
pub struct LanguageUsage {
    pub name: String,
    pub percentage: f32,
    pub color: String,
}

#[component]
pub fn LanguageBarChart(languages: Vec<LanguageUsage>) -> impl IntoView {
    view! {
        <div class="card bg-base-200">
            <div class="card-body">
                <h2 class="card-title">"言語構成"</h2>

                // スタックバー
                <div class="flex h-4 rounded-full overflow-hidden">
                    {languages
                        .iter()
                        .map(|lang| {
                            view! {
                                <div
                                    style=format!(
                                        "width: {}%; background-color: {};",
                                        lang.percentage,
                                        lang.color
                                    )
                                    title=format!("{}: {:.1}%", lang.name, lang.percentage)
                                />
                            }
                        })
                        .collect_view()}
                </div>

                // 凡例
                <div class="flex flex-wrap gap-4 mt-4">
                    {languages
                        .into_iter()
                        .map(|lang| {
                            view! {
                                <div class="flex items-center gap-2">
                                    <div
                                        class="w-3 h-3 rounded-full"
                                        style=format!("background-color: {}", lang.color)
                                    />
                                    <span class="text-sm">{lang.name}</span>
                                    <span class="text-sm text-base-content/60">
                                        {format!("{:.1}%", lang.percentage)}
                                    </span>
                                </div>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </div>
    }
}
```

### Step 6: components/mod.rs

```rust
mod contributor_pie_chart;
mod language_bar_chart;
mod repository_header;
mod top_contributors_list;

pub use contributor_pie_chart::{ContributorContribution, ContributorPieChart};
pub use language_bar_chart::{LanguageBarChart, LanguageUsage};
pub use repository_header::{RepositoryDetail, RepositoryHeader};
pub use top_contributors_list::TopContributorsList;
```

### Step 7: ページコンポーネント

`app/src/pages/repository/mod.rs`:

```rust
mod components;

use crate::components::auth_guard::AuthGuard;
use crate::components::breadcrumbs::Breadcrumbs;
use crate::components::container::Container;
use components::{
    ContributorContribution, ContributorPieChart, LanguageBarChart, LanguageUsage,
    RepositoryDetail, RepositoryHeader, TopContributorsList,
};
use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_params_map};

#[component]
pub fn RepositoryDetailPage() -> impl IntoView {
    view! {
        <AuthGuard>
            <RepositoryDetailContent />
        </AuthGuard>
    }
}

#[component]
fn RepositoryDetailContent() -> impl IntoView {
    let params = use_params_map();
    let navigate = use_navigate();

    let owner = move || params.get().get("owner").unwrap_or_default();
    let repo = move || params.get().get("repo").unwrap_or_default();

    // モックデータ
    let mock_repo = RepositoryDetail {
        name: repo(),
        description: Some("エンジニアの成長を支援するプラットフォーム".to_string()),
        url: format!("https://github.com/{}/{}", owner(), repo()),
        stars: 128,
        forks: 24,
        watchers: 15,
        language: Some("Rust".to_string()),
        language_color: Some("#dea584".to_string()),
        updated_at: "2025-11-25".to_string(),
        created_at: "2024-01-15".to_string(),
        license: Some("MIT".to_string()),
    };

    let mock_contributors = vec![
        ContributorContribution {
            username: "alice-dev".to_string(),
            avatar_url: "https://api.dicebear.com/7.x/avataaars/svg?seed=Alice".to_string(),
            contribution_count: 156,
            percentage: 45.0,
        },
        ContributorContribution {
            username: "bob-contrib".to_string(),
            avatar_url: "https://api.dicebear.com/7.x/avataaars/svg?seed=Bob".to_string(),
            contribution_count: 89,
            percentage: 25.0,
        },
        ContributorContribution {
            username: "charlie".to_string(),
            avatar_url: "https://api.dicebear.com/7.x/avataaars/svg?seed=Charlie".to_string(),
            contribution_count: 67,
            percentage: 20.0,
        },
        ContributorContribution {
            username: "others".to_string(),
            avatar_url: "https://api.dicebear.com/7.x/avataaars/svg?seed=Others".to_string(),
            contribution_count: 34,
            percentage: 10.0,
        },
    ];

    let mock_languages = vec![
        LanguageUsage {
            name: "Rust".to_string(),
            percentage: 75.0,
            color: "#dea584".to_string(),
        },
        LanguageUsage {
            name: "TypeScript".to_string(),
            percentage: 15.0,
            color: "#3178c6".to_string(),
        },
        LanguageUsage {
            name: "CSS".to_string(),
            percentage: 8.0,
            color: "#563d7c".to_string(),
        },
        LanguageUsage {
            name: "Other".to_string(),
            percentage: 2.0,
            color: "#6e7681".to_string(),
        },
    ];

    let breadcrumb_items = vec![
        ("リポジトリ".to_string(), Some("/repositories".to_string())),
        (repo(), None),
    ];

    let handle_contributor_click = move |username: String| {
        navigate(&format!("/portfolio/{}", username), Default::default());
    };

    view! {
        <Container>
            <div class="space-y-8">
                <Breadcrumbs items=breadcrumb_items />

                <RepositoryHeader repo=mock_repo />

                <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                    <ContributorPieChart contributors=mock_contributors.clone() />
                    <LanguageBarChart languages=mock_languages />
                </div>

                <TopContributorsList
                    contributors=mock_contributors
                    on_click=Callback::new(handle_contributor_click)
                />
            </div>
        </Container>
    }
}
```

### Step 8: ルーティングの追加

`app/src/lib.rs`:

```rust
use pages::RepositoryDetailPage;

// Routes 内に追加
<Route path=path!("/repository/:owner/:repo") view=RepositoryDetailPage/>
```

---

## 5. 完了条件チェックリスト

- [ ] RepositoryHeader コンポーネントが実装されている
- [ ] ContributorPieChart コンポーネントが実装されている
- [ ] TopContributorsList コンポーネントが実装されている
- [ ] LanguageBarChart コンポーネントが実装されている
- [ ] ページコンポーネントが実装されている
- [ ] 動的ルーティング（`:owner/:repo`）が設定されている
- [ ] パンくずリストが表示される
- [ ] コントリビューターをクリックするとポートフォリオに遷移する
- [ ] ビルドエラーがない

---

## 6. 参照ドキュメント

- 画面設計: `docs/02_research/2025_11/20251121_screen-design-proposal.md` - セクション 2.1.H
- PRD: `PRD.md` - セクション 5.1

