# Task 16: ダッシュボード強化

## 1. 目的と背景

### なぜこのタスクが必要か
PRDの「5.1 ダッシュボード機能」を完全に実装します。組織全体の活動サマリー、週間/月間ランキング、リアルタイムアクティビティを表示します。

### 完成時のユーザー体験
- 組織全体のKPI（コントリビューター数、リポジトリ数、PR数）が一目でわかる
- 活動量に基づくランキングが表示される
- 最近のアクティビティがリアルタイムで更新される

---

## 2. 前提条件

### 依存するタスク
- ✅ Task 5: GitHub API 実装
- ✅ Task 15: ポートフォリオ機能強化

---

## 3. 作成ファイル一覧

| ファイル | 内容 |
|---------|------|
| `app/src/pages/dashboard/components/kpi_cards.rs` | KPI カード |
| `app/src/pages/dashboard/components/ranking_card.rs` | ランキングカード |
| `app/src/pages/dashboard/components/recent_activity.rs` | 最近のアクティビティ |
| `app/src/pages/dashboard/components/repository_overview.rs` | リポジトリ概要 |

---

## 4. 実装手順

### Step 1: KPICards コンポーネント

`app/src/pages/dashboard/components/kpi_cards.rs`:

```rust
/**
 * KPICards Component
 *
 * 組織全体のKPIを表示するカード群
 */

use leptos::prelude::*;

#[derive(Debug, Clone)]
pub struct OrganizationKPI {
    pub total_contributors: i32,
    pub active_contributors_this_month: i32,
    pub total_repositories: i32,
    pub total_commits_this_month: i32,
    pub total_prs_this_month: i32,
    pub external_prs_this_month: i32,
}

#[component]
pub fn KPICards(kpi: OrganizationKPI) -> impl IntoView {
    view! {
        <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4">
            <KPICard
                title="コントリビューター"
                value=kpi.total_contributors.to_string()
                sub_value=Some(format!("今月 {} 名がアクティブ", kpi.active_contributors_this_month))
                icon="👥"
                color="primary"
            />
            <KPICard
                title="リポジトリ"
                value=kpi.total_repositories.to_string()
                sub_value=None
                icon="📁"
                color="secondary"
            />
            <KPICard
                title="コミット(今月)"
                value=kpi.total_commits_this_month.to_string()
                sub_value=None
                icon="📝"
                color="accent"
            />
            <KPICard
                title="PR(今月)"
                value=kpi.total_prs_this_month.to_string()
                sub_value=None
                icon="🔀"
                color="info"
            />
            <KPICard
                title="外部PR(今月)"
                value=kpi.external_prs_this_month.to_string()
                sub_value=None
                icon="🌐"
                color="success"
            />
            <KPICard
                title="外部貢献率"
                value=format!(
                    "{:.1}%",
                    if kpi.total_prs_this_month > 0 {
                        (kpi.external_prs_this_month as f64 / kpi.total_prs_this_month as f64) * 100.0
                    } else {
                        0.0
                    }
                )
                sub_value=None
                icon="📊"
                color="warning"
            />
        </div>
    }
}

#[component]
fn KPICard(
    title: &'static str,
    value: String,
    sub_value: Option<String>,
    icon: &'static str,
    color: &'static str,
) -> impl IntoView {
    view! {
        <div class="stat bg-base-200 rounded-lg">
            <div class="stat-figure text-2xl">{icon}</div>
            <div class="stat-title text-xs">{title}</div>
            <div class=format!("stat-value text-2xl text-{}", color)>{value}</div>
            {sub_value.map(|s| {
                view! {
                    <div class="stat-desc text-xs">{s}</div>
                }
            })}
        </div>
    }
}
```

### Step 2: RankingCard コンポーネント

`app/src/pages/dashboard/components/ranking_card.rs`:

```rust
/**
 * RankingCard Component
 */

use leptos::prelude::*;

#[derive(Debug, Clone)]
pub struct RankingEntry {
    pub rank: i32,
    pub username: String,
    pub avatar_url: String,
    pub contribution_count: i32,
    pub change: RankChange,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RankChange {
    Up(i32),
    Down(i32),
    Same,
    New,
}

impl RankChange {
    pub fn display(&self) -> String {
        match self {
            Self::Up(n) => format!("↑{}", n),
            Self::Down(n) => format!("↓{}", n),
            Self::Same => "→".to_string(),
            Self::New => "NEW".to_string(),
        }
    }

    pub fn css_class(&self) -> &'static str {
        match self {
            Self::Up(_) => "text-success",
            Self::Down(_) => "text-error",
            Self::Same => "text-base-content/60",
            Self::New => "text-warning",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RankingPeriod {
    Weekly,
    Monthly,
}

#[component]
pub fn RankingCard(
    entries: Vec<RankingEntry>,
    period: RankingPeriod,
    on_period_change: Callback<RankingPeriod>,
    #[prop(optional)] on_user_click: Option<Callback<String>>,
) -> impl IntoView {
    view! {
        <div class="card bg-base-200">
            <div class="card-body">
                <div class="flex items-center justify-between mb-4">
                    <h2 class="card-title">"🏆 アクティビティランキング"</h2>
                    <div class="tabs tabs-boxed">
                        <button
                            class=move || if period == RankingPeriod::Weekly {
                                "tab tab-active"
                            } else {
                                "tab"
                            }
                            on:click=move |_| on_period_change.call(RankingPeriod::Weekly)
                        >
                            "週間"
                        </button>
                        <button
                            class=move || if period == RankingPeriod::Monthly {
                                "tab tab-active"
                            } else {
                                "tab"
                            }
                            on:click=move |_| on_period_change.call(RankingPeriod::Monthly)
                        >
                            "月間"
                        </button>
                    </div>
                </div>

                <div class="space-y-2">
                    {entries
                        .into_iter()
                        .map(|entry| {
                            let username = entry.username.clone();
                            let callback = on_user_click.clone();

                            let handle_click = move |_| {
                                if let Some(cb) = &callback {
                                    cb.call(username.clone());
                                }
                            };

                            let rank_badge = match entry.rank {
                                1 => "🥇",
                                2 => "🥈",
                                3 => "🥉",
                                _ => "",
                            };

                            view! {
                                <div
                                    class="flex items-center gap-4 p-3 bg-base-100 rounded-lg hover:bg-base-300 cursor-pointer transition-colors"
                                    on:click=handle_click
                                >
                                    // ランク
                                    <div class="w-8 text-center font-bold">
                                        {if rank_badge.is_empty() {
                                            view! { <span>{entry.rank}</span> }.into_any()
                                        } else {
                                            view! { <span class="text-xl">{rank_badge}</span> }.into_any()
                                        }}
                                    </div>

                                    // アバター
                                    <div class="avatar">
                                        <div class="w-10 rounded-full">
                                            <img src=entry.avatar_url.clone() alt=entry.username.clone() />
                                        </div>
                                    </div>

                                    // 名前
                                    <div class="flex-1">
                                        <span class="font-medium">{entry.username.clone()}</span>
                                    </div>

                                    // コントリビューション数
                                    <div class="text-right">
                                        <div class="font-bold">{entry.contribution_count}</div>
                                        <div class=format!("text-xs {}", entry.change.css_class())>
                                            {entry.change.display()}
                                        </div>
                                    </div>
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

### Step 3: RecentActivity コンポーネント

`app/src/pages/dashboard/components/recent_activity.rs`:

```rust
/**
 * RecentActivity Component
 *
 * 組織全体の最近のアクティビティを表示
 */

use crate::concepts::contribution::{Activity, ActivityType};
use leptos::prelude::*;

#[component]
pub fn RecentActivity(
    activities: Vec<Activity>,
    #[prop(optional)] loading: bool,
) -> impl IntoView {
    view! {
        <div class="card bg-base-200">
            <div class="card-body">
                <div class="flex items-center justify-between mb-4">
                    <h2 class="card-title">"📡 最近のアクティビティ"</h2>
                    {loading.then(|| {
                        view! {
                            <span class="loading loading-dots loading-sm" />
                        }
                    })}
                </div>

                <div class="space-y-3 max-h-96 overflow-y-auto">
                    {activities
                        .into_iter()
                        .map(|activity| {
                            view! {
                                <div class="flex items-start gap-3 p-2 hover:bg-base-100 rounded-lg transition-colors">
                                    <span class="text-xl shrink-0">
                                        {activity.activity_type.icon()}
                                    </span>
                                    <div class="flex-1 min-w-0">
                                        <p class="text-sm">
                                            <span class="font-medium">{activity.title.clone()}</span>
                                        </p>
                                        <p class="text-xs text-base-content/60">
                                            {activity.repository_name.clone()}
                                            " • "
                                            {format_relative_time(&activity.created_at)}
                                        </p>
                                    </div>
                                </div>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </div>
    }
}

fn format_relative_time(date_str: &str) -> String {
    // 簡易実装
    if let Some(time_part) = date_str.split('T').nth(1) {
        if let Some(time) = time_part.split('.').next() {
            return time[..5].to_string();
        }
    }
    date_str.to_string()
}
```

### Step 4: RepositoryOverview コンポーネント

`app/src/pages/dashboard/components/repository_overview.rs`:

```rust
/**
 * RepositoryOverview Component
 */

use leptos::prelude::*;

#[derive(Debug, Clone)]
pub struct RepositorySummary {
    pub name: String,
    pub stars: i32,
    pub recent_commits: i32,
    pub language: Option<String>,
    pub language_color: Option<String>,
}

#[component]
pub fn RepositoryOverview(
    repositories: Vec<RepositorySummary>,
    #[prop(optional)] on_click: Option<Callback<String>>,
) -> impl IntoView {
    view! {
        <div class="card bg-base-200">
            <div class="card-body">
                <div class="flex items-center justify-between mb-4">
                    <h2 class="card-title">"📁 アクティブなリポジトリ"</h2>
                    <a href="/repositories" class="link link-primary text-sm">
                        "すべて見る →"
                    </a>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    {repositories
                        .into_iter()
                        .map(|repo| {
                            let name = repo.name.clone();
                            let callback = on_click.clone();

                            let handle_click = move |_| {
                                if let Some(cb) = &callback {
                                    cb.call(name.clone());
                                }
                            };

                            view! {
                                <div
                                    class="flex items-center justify-between p-4 bg-base-100 rounded-lg hover:bg-base-300 cursor-pointer transition-colors"
                                    on:click=handle_click
                                >
                                    <div class="flex items-center gap-3">
                                        {repo.language.clone().map(|lang| {
                                            let color = repo.language_color.clone().unwrap_or("#6e7681".to_string());
                                            view! {
                                                <span
                                                    class="w-3 h-3 rounded-full shrink-0"
                                                    style=format!("background-color: {}", color)
                                                />
                                            }
                                        })}
                                        <span class="font-medium">{repo.name.clone()}</span>
                                    </div>
                                    <div class="flex items-center gap-4 text-sm text-base-content/60">
                                        <span class="flex items-center gap-1">
                                            "⭐" {repo.stars}
                                        </span>
                                        <span class="flex items-center gap-1">
                                            "📝" {repo.recent_commits}
                                        </span>
                                    </div>
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

### Step 5: components/mod.rs

```rust
mod kpi_cards;
mod ranking_card;
mod recent_activity;
mod repository_overview;

pub use kpi_cards::{KPICards, OrganizationKPI};
pub use ranking_card::{RankChange, RankingCard, RankingEntry, RankingPeriod};
pub use recent_activity::RecentActivity;
pub use repository_overview::{RepositoryOverview, RepositorySummary};
```

---

## 5. 完了条件チェックリスト

- [ ] KPICards コンポーネントが実装されている
- [ ] RankingCard コンポーネントが実装されている
- [ ] RecentActivity コンポーネントが実装されている
- [ ] RepositoryOverview コンポーネントが実装されている
- [ ] ダッシュボードページに統合されている
- [ ] 週間/月間の切り替えが動作する
- [ ] ビルドエラーがない

---

## 6. 参照ドキュメント

- PRD: `PRD.md` - セクション 5.1
- 画面設計: `docs/02_research/2025_11/20251121_screen-design-proposal.md`

