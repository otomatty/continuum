# Phase 4-3: ポートフォリオページ実装

## 概要

ユーザーごとのポートフォリオページを実装し、個人のコントリビューションを可視化します。

## 目標

- ユーザープロフィール表示
- コントリビューショングラフ
- リポジトリ別フィルタ
- 時系列での活動表示

## 実装内容

### 1. ディレクトリ構造

```
app/src/pages/
└── portfolio/
    ├── mod.rs
    ├── portfolio.rs        # メインポートフォリオページ
    ├── profile.rs          # プロフィールセクション
    ├── contribution_graph.rs  # コントリビューショングラフ
    ├── repository_list.rs  # リポジトリ別コントリビューション
    └── activity_timeline.rs # 個人アクティビティタイムライン
```

### 2. ポートフォリオページ (`portfolio.rs`)

**機能:**
- ユーザー情報の取得
- コントリビューション情報の取得
- 各セクションの統合

**ルーティング:**
- `/portfolio/:username` - 特定ユーザーのポートフォリオ
- `/portfolio` - 現在ログイン中のユーザーのポートフォリオ

**実装:**
```rust
#[component]
pub fn PortfolioPage() -> impl IntoView {
    let params = use_params_map();
    let username = move || {
        params.get()
            .get("username")
            .cloned()
            .unwrap_or_else(|| "current".to_string())
    };
    
    let user_data = create_resource(
        username,
        |username| async move {
            if username == "current" {
                fetch_current_user_portfolio().await
            } else {
                fetch_user_portfolio(&username).await
            }
        }
    );
    
    view! {
        <div class="portfolio">
            <Suspense fallback=|| view! { <Loading /> }>
                {move || user_data.get().map(|data| view! {
                    <Profile user=data.user />
                    <ContributionGraph graph=data.contribution_graph />
                    <RepositoryList repos=data.repositories />
                    <ActivityTimeline activities=data.activities />
                })}
            </Suspense>
        </div>
    }
}
```

### 3. プロフィールセクション (`profile.rs`)

**表示項目:**
- ユーザー名・アバター
- プロフィール情報（bio, company, location等）
- コントリビューションサマリー
- 組織への参加日

**実装:**
```rust
#[component]
pub fn Profile(user: UserProfile) -> impl IntoView {
    view! {
        <section class="profile">
            <div class="profile-header">
                <img
                    src={user.user.avatar_url}
                    alt={format!("{}'s avatar", user.user.login)}
                    class="avatar"
                />
                <div class="profile-info">
                    <h1>{user.user.name.unwrap_or(user.user.login.clone())}</h1>
                    <p class="username">@{user.user.login}</p>
                    {user.user.bio.map(|bio| view! {
                        <p class="bio">{bio}</p>
                    })}
                    <div class="profile-meta">
                        {user.user.company.map(|company| view! {
                            <span class="meta-item">{company}</span>
                        })}
                        {user.user.location.map(|location| view! {
                            <span class="meta-item">{location}</span>
                        })}
                    </div>
                </div>
            </div>
            <div class="contribution-summary">
                <SummaryCard
                    label="Total Commits"
                    value={user.contribution_summary.total_commits}
                />
                <SummaryCard
                    label="Pull Requests"
                    value={user.contribution_summary.total_pull_requests}
                />
                <SummaryCard
                    label="Reviews"
                    value={user.contribution_summary.total_reviews}
                />
                <SummaryCard
                    label="Repositories"
                    value={user.contribution_summary.total_repositories}
                />
            </div>
        </section>
    }
}
```

### 4. コントリビューショングラフ (`contribution_graph.rs`)

**機能:**
- GitHub風のコントリビューショングラフ表示
- 期間選択（年、月、週）
- ホバーで詳細表示

**実装:**
```rust
#[component]
pub fn ContributionGraph(graph: ContributionGraph) -> impl IntoView {
    let period = create_rw_signal(ContributionPeriod::Year { year: 2024 });
    
    view! {
        <section class="contribution-graph">
            <div class="graph-header">
                <h2>Contribution Graph</h2>
                <PeriodSelector period=period />
            </div>
            <div class="graph-container">
                {graph.data
                    .iter()
                    .map(|day| view! {
                        <ContributionDay
                            day=day.clone()
                            intensity={calculate_intensity(day)}
                        />
                    })
                    .collect_view()}
            </div>
            <div class="graph-legend">
                <span>Less</span>
                <div class="legend-colors">
                    <div class="legend-color level-0"></div>
                    <div class="legend-color level-1"></div>
                    <div class="legend-color level-2"></div>
                    <div class="legend-color level-3"></div>
                    <div class="legend-color level-4"></div>
                </div>
                <span>More</span>
            </div>
        </section>
    }
}
```

### 5. リポジトリ別コントリビューション (`repository_list.rs`)

**機能:**
- ユーザーがコントリビュートしたリポジトリ一覧
- リポジトリ別のコントリビューション数
- フィルタリング・ソート

**表示項目:**
- リポジトリ名・説明
- コミット数・PR数・レビュー数
- 最終コントリビューション日

**実装:**
```rust
#[component]
pub fn RepositoryList(repos: Vec<RepositoryContribution>) -> impl IntoView {
    let filter = create_rw_signal(RepositoryFilter::All);
    let sort_by = create_rw_signal(SortBy::Contribution);
    
    view! {
        <section class="repository-list">
            <div class="list-header">
                <h2>Repositories</h2>
                <RepositoryFilter filter=filter />
                <RepositorySort sort=sort_by />
            </div>
            <div class="repository-grid">
                {repos
                    .iter()
                    .filter(|r| filter.get().matches(r))
                    .sorted_by(|a, b| sort_by.get().compare(a, b))
                    .map(|repo| view! {
                        <RepositoryContributionCard repo=repo.clone() />
                    })
                    .collect_view()}
            </div>
        </section>
    }
}
```

### 6. アクティビティタイムライン (`activity_timeline.rs`)

**機能:**
- ユーザーの個人アクティビティを時系列で表示
- アクティビティ種別フィルタ
- ページネーション

**表示項目:**
- アクティビティ種別（コミット、PR、レビュー）
- リポジトリ名
- アクティビティ詳細
- タイムスタンプ

**実装:**
```rust
#[component]
pub fn ActivityTimeline(activities: Vec<Activity>) -> impl IntoView {
    let filter = create_rw_signal(ActivityType::All);
    let page = create_rw_signal(1);
    
    view! {
        <section class="activity-timeline">
            <div class="timeline-header">
                <h2>Activity Timeline</h2>
                <ActivityTypeFilter filter=filter />
            </div>
            <div class="timeline-list">
                {activities
                    .iter()
                    .filter(|a| filter.get().matches(a))
                    .skip((page.get() - 1) * 20)
                    .take(20)
                    .map(|activity| view! {
                        <ActivityItem activity=activity.clone() />
                    })
                    .collect_view()}
            </div>
            <Pagination page=page total={activities.len()} />
        </section>
    }
}
```

### 7. API統合

**データ取得:**
```rust
async fn fetch_user_portfolio(username: &str) -> Result<PortfolioData, Error> {
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("http://localhost:3000/api/users/{}", username))
        .send()
        .await?;
    
    response.json().await
}
```

### 8. スタイリング

**レイアウト:**
- プロフィールヘッダー
- グラフ表示エリア
- リポジトリグリッド
- タイムラインリスト

**コンポーネント:**
- プロフィールカード
- コントリビューショングラフ
- リポジトリカード
- アクティビティアイテム

### 9. 共有機能

**機能:**
- ポートフォリオURLの共有
- ソーシャルメディア共有（オプション）
- 印刷用スタイル（オプション）

### 10. テスト

**テスト項目:**
- ユーザーデータ取得テスト
- グラフ表示テスト
- フィルタリング・ソートテスト
- ページネーションテスト
- 共有機能テスト

## 実装手順

1. `app/src/pages/portfolio/` ディレクトリ作成
2. `portfolio.rs` でメインページ実装
3. `profile.rs` でプロフィールセクション実装
4. `contribution_graph.rs` でグラフ実装
5. `repository_list.rs` でリポジトリ一覧実装
6. `activity_timeline.rs` でタイムライン実装
7. API統合
8. スタイリング
9. ルーティング設定
10. 共有機能実装
11. テスト作成

## 関連ドキュメント

- 親計画: `../20250101_01_setup-plan.md`
- APIエンドポイント: `../phase2-api-endpoints/README.md`
- データモデル: `../phase2-data-models/README.md`
- レイアウトコンポーネント: `../phase4-layout-components/README.md`

