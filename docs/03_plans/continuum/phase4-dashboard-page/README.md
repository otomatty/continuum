# Phase 4-2: ダッシュボードページ実装

## 概要

組織全体の活動を可視化するダッシュボードページを実装します。

## 目標

- 組織全体の活動サマリー表示
- アクティビティタイムライン
- リポジトリ一覧表示
- ランキング表示

## 実装内容

### 1. ディレクトリ構造

```
app/src/pages/
├── mod.rs
└── dashboard/
    ├── mod.rs
    ├── dashboard.rs        # メインダッシュボードページ
    ├── summary.rs          # サマリーセクション
    ├── timeline.rs         # アクティビティタイムライン
    ├── repositories.rs    # リポジトリ一覧
    └── rankings.rs          # ランキング表示
```

### 2. ダッシュボードページ (`dashboard.rs`)

**機能:**
- 各セクションの統合
- データ取得と状態管理
- ローディング状態の表示
- エラーハンドリング

**実装:**
```rust
#[component]
pub fn DashboardPage() -> impl IntoView {
    let dashboard_data = create_resource(
        || (),
        |_| async move {
            fetch_dashboard_data().await
        }
    );
    
    view! {
        <div class="dashboard">
            <h1>Dashboard</h1>
            <Suspense fallback=|| view! { <Loading /> }>
                {move || dashboard_data.get().map(|data| view! {
                    <Summary data=data.summary />
                    <Timeline activities=data.activities />
                    <Repositories repos=data.repositories />
                    <Rankings rankings=data.rankings />
                })}
            </Suspense>
        </div>
    }
}
```

### 3. サマリーセクション (`summary.rs`)

**表示項目:**
- コントリビューター数
- リポジトリ数
- 外部からのPR数
- 今週のアクティビティ数

**実装:**
```rust
#[component]
pub fn Summary(summary: DashboardSummary) -> impl IntoView {
    view! {
        <section class="summary">
            <div class="summary-grid">
                <SummaryCard
                    title="Contributors"
                    value={summary.total_contributors}
                    icon="users"
                />
                <SummaryCard
                    title="Repositories"
                    value={summary.total_repositories}
                    icon="repo"
                />
                <SummaryCard
                    title="External PRs"
                    value={summary.external_pr_count}
                    icon="git-pull-request"
                />
                <SummaryCard
                    title="This Week"
                    value={summary.this_week_activity}
                    icon="activity"
                />
            </div>
        </section>
    }
}
```

### 4. アクティビティタイムライン (`timeline.rs`)

**機能:**
- 時系列でアクティビティを表示
- フィルタリング（コミット、PR、レビュー）
- ページネーション

**表示項目:**
- アクティビティ種別（アイコン）
- ユーザー名・アバター
- リポジトリ名
- タイムスタンプ
- アクティビティ詳細

**実装:**
```rust
#[component]
pub fn Timeline(activities: Vec<Activity>) -> impl IntoView {
    let filter = create_rw_signal(ActivityFilter::All);
    
    view! {
        <section class="timeline">
            <div class="timeline-header">
                <h2>Recent Activity</h2>
                <ActivityFilter filter=filter />
            </div>
            <div class="timeline-list">
                {activities
                    .iter()
                    .filter(|a| filter.get().matches(a))
                    .map(|activity| view! {
                        <TimelineItem activity=activity.clone() />
                    })
                    .collect_view()}
            </div>
        </section>
    }
}
```

### 5. リポジトリ一覧 (`repositories.rs`)

**機能:**
- リポジトリ一覧表示
- ソート機能（Star数、更新日等）
- 検索機能
- ページネーション

**表示項目:**
- リポジトリ名・説明
- Star数・Fork数
- 言語
- 最終更新日
- コントリビューター数

**実装:**
```rust
#[component]
pub fn Repositories(repos: Vec<Repository>) -> impl IntoView {
    let sort_by = create_rw_signal(SortBy::Stars);
    let search_query = create_rw_signal(String::new());
    
    view! {
        <section class="repositories">
            <div class="repositories-header">
                <h2>Repositories</h2>
                <RepositorySearch query=search_query />
                <RepositorySort sort=sort_by />
            </div>
            <div class="repository-list">
                {repos
                    .iter()
                    .filter(|r| r.matches_search(&search_query.get()))
                    .sorted_by(|a, b| sort_by.get().compare(a, b))
                    .map(|repo| view! {
                        <RepositoryCard repo=repo.clone() />
                    })
                    .collect_view()}
            </div>
        </section>
    }
}
```

### 6. ランキング表示 (`rankings.rs`)

**機能:**
- 週間/月間ランキング
- コントリビューション数によるランキング
- リポジトリ別ランキング

**表示項目:**
- 順位
- ユーザー名・アバター
- コントリビューション数
- 前週/前月との比較

**実装:**
```rust
#[component]
pub fn Rankings(rankings: RankingsData) -> impl IntoView {
    let period = create_rw_signal(RankingPeriod::Week);
    
    view! {
        <section class="rankings">
            <div class="rankings-header">
                <h2>Top Contributors</h2>
                <PeriodSelector period=period />
            </div>
            <div class="rankings-list">
                {rankings
                    .get_rankings(period.get())
                    .iter()
                    .enumerate()
                    .map(|(index, user)| view! {
                        <RankingItem
                            rank={index + 1}
                            user=user.clone()
                        />
                    })
                    .collect_view()}
            </div>
        </section>
    }
}
```

### 7. API統合

**データ取得:**
```rust
async fn fetch_dashboard_data() -> Result<DashboardData, Error> {
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:3000/api/dashboard")
        .send()
        .await?;
    
    response.json().await
}
```

### 8. スタイリング

**レイアウト:**
- グリッドレイアウト
- カードベースのデザイン
- レスポンシブ対応

**コンポーネント:**
- サマリーカード
- タイムラインアイテム
- リポジトリカード
- ランキングアイテム

### 9. テスト

**テスト項目:**
- データ取得テスト
- フィルタリングテスト
- ソート機能テスト
- ページネーションテスト
- エラーハンドリングテスト

## 実装手順

1. `app/src/pages/dashboard/` ディレクトリ作成
2. `dashboard.rs` でメインページ実装
3. `summary.rs` でサマリーセクション実装
4. `timeline.rs` でタイムライン実装
5. `repositories.rs` でリポジトリ一覧実装
6. `rankings.rs` でランキング実装
7. API統合
8. スタイリング
9. ルーティング設定
10. テスト作成

## 関連ドキュメント

- 親計画: `../20250101_01_setup-plan.md`
- APIエンドポイント: `../phase2-api-endpoints/README.md`
- レイアウトコンポーネント: `../phase4-layout-components/README.md`

