# Task 15: ポートフォリオ機能強化

## 1. 目的と背景

### なぜこのタスクが必要か
PRDの「5.3 ポートフォリオ機能」に記載されている機能を完全に実装します。ユーザーの全コントリビューション（コミット、PR、レビュー等）を時系列やリポジトリ別に可視化し、社外にも共有可能な公開URLを提供します。

### 完成時のユーザー体験
- コントリビューションカレンダー（GitHub の草グラフ）が表示される
- リポジトリ別のコントリビューション統計が表示される
- 時系列のアクティビティタイムラインが表示される
- 外部共有用の公開URLが生成される

---

## 2. 前提条件

### 依存するタスク
- ✅ Task 5: GitHub API 実装
- ✅ Task 7: コントリビューター一覧ページ

---

## 3. 作成/更新ファイル一覧

### 新規作成ファイル
| ファイル | 内容 |
|---------|------|
| `app/src/pages/portfolio/components/contribution_calendar.rs` | コントリビューションカレンダー |
| `app/src/pages/portfolio/components/repository_stats.rs` | リポジトリ別統計 |
| `app/src/pages/portfolio/components/activity_timeline.rs` | アクティビティタイムライン |
| `app/src/pages/portfolio/components/share_button.rs` | 共有ボタン |
| `app/src/concepts/contribution/state.rs` | コントリビューション状態 |
| `app/src/concepts/contribution/actions.rs` | コントリビューション操作 |

### 更新ファイル
| ファイル | 変更内容 |
|---------|---------|
| `app/src/pages/portfolio/mod.rs` | 新コンポーネントの統合 |
| `app/src/github/queries.rs` | コントリビューション取得クエリ追加 |

---

## 4. 実装手順

### Step 1: Contribution Concept の実装

`app/src/concepts/contribution/state.rs`:

```rust
use serde::{Deserialize, Serialize};

/// コントリビューションの状態
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContributionState {
    /// コントリビューションカレンダーデータ
    pub calendar: Option<ContributionCalendar>,
    /// リポジトリ別の統計
    pub repository_stats: Vec<RepositoryContribution>,
    /// アクティビティタイムライン
    pub activities: Vec<Activity>,
    /// 読み込み中フラグ
    pub loading: bool,
}

/// コントリビューションカレンダー（1年分）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributionCalendar {
    pub total_contributions: i32,
    pub weeks: Vec<ContributionWeek>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributionWeek {
    pub days: Vec<ContributionDay>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributionDay {
    pub date: String,
    pub count: i32,
    pub level: ContributionLevel,
}

/// コントリビューションの濃さ（GitHub の草の色に対応）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContributionLevel {
    None,   // 0
    Low,    // 1-3
    Medium, // 4-6
    High,   // 7-9
    Max,    // 10+
}

impl ContributionLevel {
    pub fn from_count(count: i32) -> Self {
        match count {
            0 => Self::None,
            1..=3 => Self::Low,
            4..=6 => Self::Medium,
            7..=9 => Self::High,
            _ => Self::Max,
        }
    }

    pub fn css_class(&self) -> &'static str {
        match self {
            Self::None => "bg-base-300",
            Self::Low => "bg-success/25",
            Self::Medium => "bg-success/50",
            Self::High => "bg-success/75",
            Self::Max => "bg-success",
        }
    }
}

/// リポジトリ別のコントリビューション
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryContribution {
    pub repository_name: String,
    pub repository_url: String,
    pub commits: i32,
    pub pull_requests: i32,
    pub reviews: i32,
    pub issues: i32,
    pub total: i32,
}

/// アクティビティ（タイムライン用）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub id: String,
    pub activity_type: ActivityType,
    pub title: String,
    pub repository_name: String,
    pub url: String,
    pub created_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActivityType {
    Commit,
    PullRequest,
    PullRequestReview,
    Issue,
    IssueComment,
}

impl ActivityType {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Commit => "📝",
            Self::PullRequest => "🔀",
            Self::PullRequestReview => "👀",
            Self::Issue => "📌",
            Self::IssueComment => "💬",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Commit => "コミット",
            Self::PullRequest => "プルリクエスト",
            Self::PullRequestReview => "レビュー",
            Self::Issue => "Issue",
            Self::IssueComment => "コメント",
        }
    }
}
```

### Step 2: ContributionCalendar コンポーネント

`app/src/pages/portfolio/components/contribution_calendar.rs`:

```rust
/**
 * ContributionCalendar Component
 *
 * GitHub の草グラフを再現したコントリビューションカレンダー
 */

use crate::concepts::contribution::{ContributionCalendar as CalendarData, ContributionLevel};
use leptos::prelude::*;

#[component]
pub fn ContributionCalendar(calendar: CalendarData) -> impl IntoView {
    view! {
        <div class="card bg-base-200">
            <div class="card-body">
                <div class="flex items-center justify-between mb-4">
                    <h2 class="card-title">"コントリビューション"</h2>
                    <span class="text-2xl font-bold text-success">
                        {calendar.total_contributions}
                    </span>
                </div>

                // カレンダーグリッド
                <div class="overflow-x-auto pb-2">
                    <div class="flex gap-1" style="min-width: 720px;">
                        {calendar.weeks
                            .into_iter()
                            .map(|week| {
                                view! {
                                    <div class="flex flex-col gap-1">
                                        {week.days
                                            .into_iter()
                                            .map(|day| {
                                                view! {
                                                    <div
                                                        class=format!(
                                                            "w-3 h-3 rounded-sm {} tooltip",
                                                            day.level.css_class()
                                                        )
                                                        data-tip=format!("{}: {} contributions", day.date, day.count)
                                                    />
                                                }
                                            })
                                            .collect_view()}
                                    </div>
                                }
                            })
                            .collect_view()}
                    </div>
                </div>

                // 凡例
                <div class="flex items-center justify-end gap-2 mt-4 text-sm">
                    <span class="text-base-content/60">"Less"</span>
                    <div class=format!("w-3 h-3 rounded-sm {}", ContributionLevel::None.css_class()) />
                    <div class=format!("w-3 h-3 rounded-sm {}", ContributionLevel::Low.css_class()) />
                    <div class=format!("w-3 h-3 rounded-sm {}", ContributionLevel::Medium.css_class()) />
                    <div class=format!("w-3 h-3 rounded-sm {}", ContributionLevel::High.css_class()) />
                    <div class=format!("w-3 h-3 rounded-sm {}", ContributionLevel::Max.css_class()) />
                    <span class="text-base-content/60">"More"</span>
                </div>
            </div>
        </div>
    }
}
```

### Step 3: RepositoryStats コンポーネント

`app/src/pages/portfolio/components/repository_stats.rs`:

```rust
/**
 * RepositoryStats Component
 */

use crate::concepts::contribution::RepositoryContribution;
use leptos::prelude::*;

#[component]
pub fn RepositoryStats(stats: Vec<RepositoryContribution>) -> impl IntoView {
    view! {
        <div class="card bg-base-200">
            <div class="card-body">
                <h2 class="card-title">"リポジトリ別コントリビューション"</h2>

                <div class="overflow-x-auto">
                    <table class="table">
                        <thead>
                            <tr>
                                <th>"リポジトリ"</th>
                                <th class="text-right">"コミット"</th>
                                <th class="text-right">"PR"</th>
                                <th class="text-right">"レビュー"</th>
                                <th class="text-right">"Issue"</th>
                                <th class="text-right">"合計"</th>
                            </tr>
                        </thead>
                        <tbody>
                            {stats
                                .into_iter()
                                .map(|stat| {
                                    view! {
                                        <tr class="hover">
                                            <td>
                                                <a
                                                    href=stat.repository_url.clone()
                                                    target="_blank"
                                                    rel="noopener noreferrer"
                                                    class="link link-hover"
                                                >
                                                    {stat.repository_name.clone()}
                                                </a>
                                            </td>
                                            <td class="text-right">{stat.commits}</td>
                                            <td class="text-right">{stat.pull_requests}</td>
                                            <td class="text-right">{stat.reviews}</td>
                                            <td class="text-right">{stat.issues}</td>
                                            <td class="text-right font-bold">{stat.total}</td>
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

### Step 4: ActivityTimeline コンポーネント

`app/src/pages/portfolio/components/activity_timeline.rs`:

```rust
/**
 * ActivityTimeline Component
 */

use crate::concepts::contribution::Activity;
use leptos::prelude::*;

#[component]
pub fn ActivityTimeline(activities: Vec<Activity>) -> impl IntoView {
    view! {
        <div class="card bg-base-200">
            <div class="card-body">
                <h2 class="card-title">"最近のアクティビティ"</h2>

                <ul class="timeline timeline-vertical timeline-compact">
                    {activities
                        .into_iter()
                        .map(|activity| {
                            let formatted_date = format_relative_time(&activity.created_at);

                            view! {
                                <li>
                                    <div class="timeline-start text-sm text-base-content/60">
                                        {formatted_date}
                                    </div>
                                    <div class="timeline-middle">
                                        <span class="text-lg">{activity.activity_type.icon()}</span>
                                    </div>
                                    <div class="timeline-end timeline-box">
                                        <div class="font-medium">
                                            {activity.activity_type.label()}
                                        </div>
                                        <a
                                            href=activity.url.clone()
                                            target="_blank"
                                            rel="noopener noreferrer"
                                            class="link link-hover text-sm"
                                        >
                                            {activity.title.clone()}
                                        </a>
                                        <div class="text-xs text-base-content/60">
                                            {activity.repository_name.clone()}
                                        </div>
                                    </div>
                                    <hr />
                                </li>
                            }
                        })
                        .collect_view()}
                </ul>
            </div>
        </div>
    }
}

/// ISO 8601 日付から相対時間文字列を生成
fn format_relative_time(date_str: &str) -> String {
    // 簡易実装：日付部分だけ表示
    if let Some(date_part) = date_str.split('T').next() {
        date_part.to_string()
    } else {
        date_str.to_string()
    }
}
```

### Step 5: ShareButton コンポーネント

`app/src/pages/portfolio/components/share_button.rs`:

```rust
/**
 * ShareButton Component
 */

use leptos::prelude::*;

#[component]
pub fn ShareButton(username: String) -> impl IntoView {
    let (copied, set_copied) = signal(false);

    let share_url = format!("{}/portfolio/{}", 
        // TODO: 環境変数から取得
        "https://continuum.example.com",
        username
    );

    let handle_copy = move |_| {
        #[cfg(feature = "hydrate")]
        {
            use wasm_bindgen::prelude::*;
            use web_sys::window;

            if let Some(window) = window() {
                if let Some(navigator) = window.navigator().clipboard() {
                    let url = share_url.clone();
                    let future = wasm_bindgen_futures::JsFuture::from(
                        navigator.write_text(&url)
                    );
                    leptos::task::spawn_local(async move {
                        if future.await.is_ok() {
                            set_copied.set(true);
                            // 2秒後にリセット
                            gloo_timers::future::TimeoutFuture::new(2000).await;
                            set_copied.set(false);
                        }
                    });
                }
            }
        }
    };

    view! {
        <div class="dropdown dropdown-end">
            <div tabindex="0" role="button" class="btn btn-outline btn-sm gap-2">
                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.368 2.684 3 3 0 00-5.368-2.684z" />
                </svg>
                "共有"
            </div>
            <ul tabindex="0" class="dropdown-content z-[1] menu p-4 shadow bg-base-200 rounded-box w-80">
                <li class="mb-2">
                    <div class="text-sm font-medium mb-1">"公開URL"</div>
                    <div class="flex gap-2">
                        <input
                            type="text"
                            class="input input-bordered input-sm flex-1"
                            readonly
                            value=share_url.clone()
                        />
                        <button
                            class="btn btn-sm btn-primary"
                            on:click=handle_copy
                        >
                            {move || if copied.get() {
                                "✓"
                            } else {
                                "📋"
                            }}
                        </button>
                    </div>
                </li>
            </ul>
        </div>
    }
}
```

---

## 5. 完了条件チェックリスト

- [ ] Contribution Concept が実装されている
- [ ] ContributionCalendar コンポーネントが実装されている
- [ ] RepositoryStats コンポーネントが実装されている
- [ ] ActivityTimeline コンポーネントが実装されている
- [ ] ShareButton コンポーネントが実装されている
- [ ] ポートフォリオページに統合されている
- [ ] 公開URLでアクセスできる
- [ ] ビルドエラーがない

---

## 6. 参照ドキュメント

- PRD: `PRD.md` - セクション 5.3
- 画面設計: `docs/02_research/2025_11/20251121_screen-design-proposal.md`

