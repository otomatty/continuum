# Task 18: アクティビティランキング

## 1. 目的と背景

### なぜこのタスクが必要か
PRDの「5.1 ダッシュボード機能」に記載されている「活動量に基づく週間/月間ランキング」を完全に実装します。Task 16で基本的なUIを作成しましたが、このタスクではロジックとデータ取得を実装します。

### 完成時のユーザー体験
- 週間/月間の切り替えでランキングが更新される
- ランキングの変動（前週/前月比）が表示される
- トップ10以外も「もっと見る」で確認できる

---

## 2. 前提条件

### 依存するタスク
- ✅ Task 5: GitHub API 実装
- ✅ Task 16: ダッシュボード強化

---

## 3. 作成ファイル一覧

| ファイル | 内容 |
|---------|------|
| `app/src/concepts/ranking/state.rs` | ランキング状態 |
| `app/src/concepts/ranking/actions.rs` | ランキング計算ロジック |
| `app/src/concepts/ranking/ranking.spec.md` | 仕様書 |
| `app/src/concepts/ranking/tests.rs` | テスト |
| `app/src/concepts/ranking/mod.rs` | モジュール定義 |
| `app/src/synchronizations/ranking_sync.rs` | ランキング計算同期 |

---

## 4. 実装手順

### Step 1: Ranking Concept の状態定義

`app/src/concepts/ranking/state.rs`:

```rust
use serde::{Deserialize, Serialize};

/// ランキングの状態
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RankingState {
    /// 現在の期間
    pub period: RankingPeriod,
    /// ランキングエントリ
    pub entries: Vec<RankingEntry>,
    /// 前回の期間のランキング（変動計算用）
    pub previous_entries: Vec<RankingEntry>,
    /// 読み込み中フラグ
    pub loading: bool,
    /// エラー
    pub error: Option<String>,
}

/// ランキング期間
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub enum RankingPeriod {
    #[default]
    Weekly,
    Monthly,
}

impl RankingPeriod {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Weekly => "週間",
            Self::Monthly => "月間",
        }
    }

    pub fn days(&self) -> i64 {
        match self {
            Self::Weekly => 7,
            Self::Monthly => 30,
        }
    }
}

/// ランキングエントリ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankingEntry {
    /// ユーザーID
    pub user_id: String,
    /// ユーザー名
    pub username: String,
    /// アバターURL
    pub avatar_url: String,
    /// コントリビューション数
    pub contribution_count: i32,
    /// コミット数
    pub commits: i32,
    /// PR数
    pub pull_requests: i32,
    /// レビュー数
    pub reviews: i32,
    /// Issue数
    pub issues: i32,
}

/// ランキング変動
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RankChange {
    /// 順位上昇
    Up(i32),
    /// 順位下降
    Down(i32),
    /// 変動なし
    Same,
    /// 新規ランクイン
    New,
}

impl RankChange {
    pub fn css_class(&self) -> &'static str {
        match self {
            Self::Up(_) => "text-success",
            Self::Down(_) => "text-error",
            Self::Same => "text-base-content/60",
            Self::New => "badge badge-warning badge-sm",
        }
    }

    pub fn display(&self) -> String {
        match self {
            Self::Up(n) => format!("↑{}", n),
            Self::Down(n) => format!("↓{}", n),
            Self::Same => "→".to_string(),
            Self::New => "NEW".to_string(),
        }
    }
}
```

### Step 2: Ranking Concept のアクション定義

`app/src/concepts/ranking/actions.rs`:

```rust
use super::state::*;

/// ランキングエントリを設定
pub fn set_entries(state: RankingState, entries: Vec<RankingEntry>) -> RankingState {
    RankingState {
        entries,
        loading: false,
        error: None,
        ..state
    }
}

/// 前回のエントリを設定
pub fn set_previous_entries(
    state: RankingState,
    previous_entries: Vec<RankingEntry>,
) -> RankingState {
    RankingState {
        previous_entries,
        ..state
    }
}

/// 期間を変更
pub fn set_period(state: RankingState, period: RankingPeriod) -> RankingState {
    RankingState { period, ..state }
}

/// 読み込み開始
pub fn set_loading(state: RankingState, loading: bool) -> RankingState {
    RankingState { loading, ..state }
}

/// エラーを設定
pub fn set_error(state: RankingState, error: String) -> RankingState {
    RankingState {
        error: Some(error),
        loading: false,
        ..state
    }
}

/// ランキング変動を計算
pub fn calculate_rank_change(
    current_entries: &[RankingEntry],
    previous_entries: &[RankingEntry],
    user_id: &str,
) -> RankChange {
    // 現在の順位を取得
    let current_rank = current_entries
        .iter()
        .position(|e| e.user_id == user_id)
        .map(|i| i + 1);

    // 前回の順位を取得
    let previous_rank = previous_entries
        .iter()
        .position(|e| e.user_id == user_id)
        .map(|i| i + 1);

    match (current_rank, previous_rank) {
        (Some(current), Some(previous)) => {
            if current < previous {
                RankChange::Up((previous - current) as i32)
            } else if current > previous {
                RankChange::Down((current - previous) as i32)
            } else {
                RankChange::Same
            }
        }
        (Some(_), None) => RankChange::New, // 新規ランクイン
        _ => RankChange::Same,
    }
}

/// エントリをスコアでソート
pub fn sort_by_contribution(mut entries: Vec<RankingEntry>) -> Vec<RankingEntry> {
    entries.sort_by(|a, b| b.contribution_count.cmp(&a.contribution_count));
    entries
}

/// 上位N件を取得
pub fn take_top_n(entries: Vec<RankingEntry>, n: usize) -> Vec<RankingEntry> {
    entries.into_iter().take(n).collect()
}

/// コントリビューション数を集計
pub fn calculate_contribution_count(entry: &RankingEntry) -> i32 {
    entry.commits + entry.pull_requests * 2 + entry.reviews + entry.issues
}
```

### Step 3: Ranking Synchronization

`app/src/synchronizations/ranking_sync.rs`:

```rust
/**
 * Ranking Synchronization
 *
 * DEPENDENCY MAP:
 * Concepts:
 *   ├─ src/concepts/ranking/mod.rs
 *   └─ src/concepts/contribution/mod.rs
 * Related Documentation:
 *   └─ Spec: ./ranking.spec.md
 */

use crate::concepts::contribution::ContributionState;
use crate::concepts::ranking::{
    actions::*, state::*, RankingEntry, RankingPeriod, RankingState,
};

/// when: コントリビューションデータが更新されたら
/// then: ランキングを再計算する
pub fn on_contribution_updated(
    ranking_state: RankingState,
    contributions: &[UserContributionSummary],
) -> RankingState {
    let entries: Vec<RankingEntry> = contributions
        .iter()
        .map(|c| RankingEntry {
            user_id: c.user_id.clone(),
            username: c.username.clone(),
            avatar_url: c.avatar_url.clone(),
            contribution_count: c.commits + c.pull_requests * 2 + c.reviews + c.issues,
            commits: c.commits,
            pull_requests: c.pull_requests,
            reviews: c.reviews,
            issues: c.issues,
        })
        .collect();

    let sorted_entries = sort_by_contribution(entries);
    set_entries(ranking_state, sorted_entries)
}

/// ユーザーごとのコントリビューションサマリー
#[derive(Debug, Clone)]
pub struct UserContributionSummary {
    pub user_id: String,
    pub username: String,
    pub avatar_url: String,
    pub commits: i32,
    pub pull_requests: i32,
    pub reviews: i32,
    pub issues: i32,
}
```

### Step 4: テストの作成

`app/src/concepts/ranking/tests.rs`:

```rust
#[cfg(test)]
mod tests {
    use crate::concepts::ranking::actions::*;
    use crate::concepts::ranking::state::*;

    fn create_test_entry(user_id: &str, count: i32) -> RankingEntry {
        RankingEntry {
            user_id: user_id.to_string(),
            username: format!("user-{}", user_id),
            avatar_url: format!("https://example.com/{}.png", user_id),
            contribution_count: count,
            commits: count / 2,
            pull_requests: count / 4,
            reviews: count / 8,
            issues: count / 8,
        }
    }

    #[test]
    fn test_set_entries() {
        let state = RankingState::default();
        let entries = vec![create_test_entry("1", 100)];
        
        let new_state = set_entries(state, entries);
        
        assert_eq!(new_state.entries.len(), 1);
        assert!(!new_state.loading);
    }

    #[test]
    fn test_sort_by_contribution() {
        let entries = vec![
            create_test_entry("1", 50),
            create_test_entry("2", 100),
            create_test_entry("3", 75),
        ];
        
        let sorted = sort_by_contribution(entries);
        
        assert_eq!(sorted[0].user_id, "2");
        assert_eq!(sorted[1].user_id, "3");
        assert_eq!(sorted[2].user_id, "1");
    }

    #[test]
    fn test_calculate_rank_change_up() {
        let current = vec![
            create_test_entry("1", 100),
            create_test_entry("2", 90),
        ];
        let previous = vec![
            create_test_entry("2", 100),
            create_test_entry("1", 90),
        ];
        
        let change = calculate_rank_change(&current, &previous, "1");
        
        assert_eq!(change, RankChange::Up(1));
    }

    #[test]
    fn test_calculate_rank_change_new() {
        let current = vec![
            create_test_entry("1", 100),
            create_test_entry("2", 90),
        ];
        let previous = vec![
            create_test_entry("1", 100),
        ];
        
        let change = calculate_rank_change(&current, &previous, "2");
        
        assert_eq!(change, RankChange::New);
    }

    #[test]
    fn test_take_top_n() {
        let entries = vec![
            create_test_entry("1", 100),
            create_test_entry("2", 90),
            create_test_entry("3", 80),
        ];
        
        let top_2 = take_top_n(entries, 2);
        
        assert_eq!(top_2.len(), 2);
    }
}
```

---

## 5. 完了条件チェックリスト

- [ ] Ranking Concept が実装されている
  - [ ] state.rs
  - [ ] actions.rs
  - [ ] tests.rs
  - [ ] mod.rs
- [ ] ranking_sync.rs が実装されている
- [ ] ランキング計算ロジックが正しく動作する
- [ ] 週間/月間の切り替えが動作する
- [ ] ランキング変動が正しく計算される
- [ ] 全テストが通る
- [ ] ビルドエラーがない

---

## 6. 参照ドキュメント

- PRD: `PRD.md` - セクション 5.1
- Task 16: `task-16-dashboard-enhancement.md`
- Legible Architecture: ワークスペースの `.cursor/rules` 参照

