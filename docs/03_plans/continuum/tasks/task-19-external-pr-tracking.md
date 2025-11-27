# Task 19: 外部PR追跡

## 1. 目的と背景

### なぜこのタスクが必要か
PRDの「2.1 背景と目的」に記載されている「オープンな文化の醸成」の一環として、組織メンバー以外（元社員、外部コントリビューター）からのPRを追跡・可視化します。

### 完成時のユーザー体験
- ダッシュボードで外部PRの数が表示される
- 外部コントリビューターの一覧が確認できる
- 元社員からの継続的な貢献が可視化される

---

## 2. 前提条件

### 依存するタスク
- ✅ Task 5: GitHub API 実装
- ✅ Task 7: コントリビューター一覧ページ

---

## 3. 作成ファイル一覧

| ファイル | 内容 |
|---------|------|
| `app/src/concepts/external_contributor/state.rs` | 外部コントリビューター状態 |
| `app/src/concepts/external_contributor/actions.rs` | 外部コントリビューター操作 |
| `app/src/concepts/external_contributor/mod.rs` | モジュール定義 |
| `app/src/pages/dashboard/components/external_pr_card.rs` | 外部PRカード |
| `app/src/github/queries.rs` | 外部PR取得クエリ追加 |

---

## 4. 実装手順

### Step 1: External Contributor Concept の状態定義

`app/src/concepts/external_contributor/state.rs`:

```rust
use serde::{Deserialize, Serialize};

/// 外部コントリビューターの状態
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExternalContributorState {
    /// 外部コントリビューター一覧
    pub contributors: Vec<ExternalContributor>,
    /// 外部PRの統計
    pub stats: ExternalPRStats,
    /// 読み込み中フラグ
    pub loading: bool,
}

/// 外部コントリビューター
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContributor {
    pub username: String,
    pub avatar_url: String,
    /// 外部コントリビューターのタイプ
    pub contributor_type: ContributorType,
    /// 総PR数
    pub total_prs: i32,
    /// マージされたPR数
    pub merged_prs: i32,
    /// 最終貢献日
    pub last_contribution_at: String,
    /// 貢献しているリポジトリ
    pub contributed_repositories: Vec<String>,
}

/// コントリビューターのタイプ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContributorType {
    /// 元社員（アルムナイ）
    Alumni,
    /// 外部コントリビューター
    External,
}

impl ContributorType {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Alumni => "アルムナイ",
            Self::External => "外部",
        }
    }

    pub fn badge_class(&self) -> &'static str {
        match self {
            Self::Alumni => "badge-info",
            Self::External => "badge-warning",
        }
    }
}

/// 外部PRの統計
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExternalPRStats {
    /// 今月の外部PR数
    pub this_month: i32,
    /// 先月の外部PR数
    pub last_month: i32,
    /// 今月のマージされた外部PR数
    pub merged_this_month: i32,
    /// 外部コントリビューター数
    pub unique_contributors: i32,
}

impl ExternalPRStats {
    /// 前月比の変化率を計算
    pub fn change_percentage(&self) -> f64 {
        if self.last_month == 0 {
            if self.this_month > 0 {
                100.0
            } else {
                0.0
            }
        } else {
            ((self.this_month - self.last_month) as f64 / self.last_month as f64) * 100.0
        }
    }

    /// 変化がポジティブかどうか
    pub fn is_positive_change(&self) -> bool {
        self.this_month >= self.last_month
    }
}
```

### Step 2: External Contributor Concept のアクション定義

`app/src/concepts/external_contributor/actions.rs`:

```rust
use super::state::*;

/// 外部コントリビューター一覧を設定
pub fn set_contributors(
    state: ExternalContributorState,
    contributors: Vec<ExternalContributor>,
) -> ExternalContributorState {
    ExternalContributorState {
        contributors,
        loading: false,
        ..state
    }
}

/// 統計を設定
pub fn set_stats(
    state: ExternalContributorState,
    stats: ExternalPRStats,
) -> ExternalContributorState {
    ExternalContributorState { stats, ..state }
}

/// 読み込み開始
pub fn set_loading(state: ExternalContributorState, loading: bool) -> ExternalContributorState {
    ExternalContributorState { loading, ..state }
}

/// 組織メンバーかどうかを判定
pub fn is_organization_member(member_logins: &[String], username: &str) -> bool {
    member_logins.iter().any(|m| m == username)
}

/// PR一覧から外部コントリビューターを抽出
pub fn extract_external_contributors(
    prs: Vec<PullRequestInfo>,
    member_logins: &[String],
) -> Vec<ExternalContributor> {
    use std::collections::HashMap;

    let mut contributor_map: HashMap<String, ExternalContributor> = HashMap::new();

    for pr in prs {
        if is_organization_member(member_logins, &pr.author) {
            continue;
        }

        let entry = contributor_map
            .entry(pr.author.clone())
            .or_insert_with(|| ExternalContributor {
                username: pr.author.clone(),
                avatar_url: pr.author_avatar_url.clone(),
                contributor_type: ContributorType::External, // TODO: アルムナイ判定
                total_prs: 0,
                merged_prs: 0,
                last_contribution_at: pr.created_at.clone(),
                contributed_repositories: vec![],
            });

        entry.total_prs += 1;
        if pr.merged {
            entry.merged_prs += 1;
        }
        
        if !entry.contributed_repositories.contains(&pr.repository) {
            entry.contributed_repositories.push(pr.repository.clone());
        }

        // 最終貢献日を更新
        if pr.created_at > entry.last_contribution_at {
            entry.last_contribution_at = pr.created_at.clone();
        }
    }

    contributor_map.into_values().collect()
}

/// 外部PR統計を計算
pub fn calculate_external_pr_stats(
    external_prs_this_month: i32,
    external_prs_last_month: i32,
    merged_this_month: i32,
    unique_contributors: i32,
) -> ExternalPRStats {
    ExternalPRStats {
        this_month: external_prs_this_month,
        last_month: external_prs_last_month,
        merged_this_month,
        unique_contributors,
    }
}

/// PR情報（内部用）
#[derive(Debug, Clone)]
pub struct PullRequestInfo {
    pub author: String,
    pub author_avatar_url: String,
    pub repository: String,
    pub merged: bool,
    pub created_at: String,
}
```

### Step 3: ExternalPRCard コンポーネント

`app/src/pages/dashboard/components/external_pr_card.rs`:

```rust
/**
 * ExternalPRCard Component
 *
 * 外部PRの統計を表示するカード
 */

use crate::concepts::external_contributor::{ExternalContributor, ExternalPRStats};
use leptos::prelude::*;

#[component]
pub fn ExternalPRCard(
    stats: ExternalPRStats,
    top_contributors: Vec<ExternalContributor>,
    #[prop(optional)] on_view_all: Option<Callback<()>>,
) -> impl IntoView {
    let change_percentage = stats.change_percentage();
    let is_positive = stats.is_positive_change();

    view! {
        <div class="card bg-base-200">
            <div class="card-body">
                <div class="flex items-center justify-between mb-4">
                    <h2 class="card-title">"🌐 外部コントリビューション"</h2>
                    {on_view_all.map(|cb| {
                        view! {
                            <button
                                class="btn btn-ghost btn-sm"
                                on:click=move |_| cb.call(())
                            >
                                "すべて見る →"
                            </button>
                        }
                    })}
                </div>

                // 統計
                <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
                    <div class="text-center">
                        <div class="text-3xl font-bold text-primary">
                            {stats.this_month}
                        </div>
                        <div class="text-sm text-base-content/60">"今月のPR"</div>
                    </div>
                    <div class="text-center">
                        <div class=format!(
                            "text-lg font-bold {}",
                            if is_positive { "text-success" } else { "text-error" }
                        )>
                            {if is_positive { "↑" } else { "↓" }}
                            {format!("{:.1}%", change_percentage.abs())}
                        </div>
                        <div class="text-sm text-base-content/60">"前月比"</div>
                    </div>
                    <div class="text-center">
                        <div class="text-3xl font-bold text-success">
                            {stats.merged_this_month}
                        </div>
                        <div class="text-sm text-base-content/60">"マージ済み"</div>
                    </div>
                    <div class="text-center">
                        <div class="text-3xl font-bold text-secondary">
                            {stats.unique_contributors}
                        </div>
                        <div class="text-sm text-base-content/60">"コントリビューター"</div>
                    </div>
                </div>

                // トップコントリビューター
                {(!top_contributors.is_empty()).then(|| {
                    view! {
                        <div>
                            <h3 class="font-medium mb-3">"トップ外部コントリビューター"</h3>
                            <div class="space-y-2">
                                {top_contributors
                                    .into_iter()
                                    .take(5)
                                    .map(|contributor| {
                                        view! {
                                            <div class="flex items-center justify-between p-2 bg-base-100 rounded-lg">
                                                <div class="flex items-center gap-3">
                                                    <div class="avatar">
                                                        <div class="w-8 rounded-full">
                                                            <img
                                                                src=contributor.avatar_url.clone()
                                                                alt=contributor.username.clone()
                                                            />
                                                        </div>
                                                    </div>
                                                    <span class="font-medium">
                                                        {contributor.username.clone()}
                                                    </span>
                                                    <span class=format!(
                                                        "badge badge-sm {}",
                                                        contributor.contributor_type.badge_class()
                                                    )>
                                                        {contributor.contributor_type.label()}
                                                    </span>
                                                </div>
                                                <div class="text-sm">
                                                    <span class="font-bold">{contributor.merged_prs}</span>
                                                    <span class="text-base-content/60">" / "</span>
                                                    <span>{contributor.total_prs}</span>
                                                    <span class="text-base-content/60">" PRs"</span>
                                                </div>
                                            </div>
                                        }
                                    })
                                    .collect_view()}
                            </div>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}
```

### Step 4: GraphQL クエリの追加

`app/src/github/queries.rs` に追加：

```rust
/// 外部PRを取得するクエリ
pub const EXTERNAL_PRS_QUERY: &str = r#"
query ExternalPRs($org: String!, $since: DateTime!) {
  organization(login: $org) {
    repositories(first: 100, privacy: PUBLIC) {
      nodes {
        name
        pullRequests(
          first: 100
          states: [OPEN, MERGED]
          orderBy: {field: CREATED_AT, direction: DESC}
        ) {
          nodes {
            author {
              login
              avatarUrl
            }
            merged
            createdAt
          }
        }
      }
    }
    membersWithRole(first: 100) {
      nodes {
        login
      }
    }
  }
}
"#;
```

---

## 5. 完了条件チェックリスト

- [ ] ExternalContributor Concept が実装されている
- [ ] ExternalPRCard コンポーネントが実装されている
- [ ] GraphQL クエリが追加されている
- [ ] 外部コントリビューターの判定ロジックが正しく動作する
- [ ] 統計が正しく計算される
- [ ] ダッシュボードに統合されている
- [ ] ビルドエラーがない

---

## 6. 参照ドキュメント

- PRD: `PRD.md` - セクション 2.1
- Task 16: `task-16-dashboard-enhancement.md`

---

## 7. 注意点

- **アルムナイ判定**: 元社員の判定は別途データソース（退職者リストなど）が必要
- **パフォーマンス**: 大量のPRがある場合はページネーションを考慮
- **プライバシー**: 外部コントリビューターの情報は公開情報のみを使用

