/**
 * Discussion Concept - State
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   ├─ src/concepts/discussion/mod.rs
 *   ├─ src/concepts/discussion/actions.rs
 *   └─ src/concepts/discussion/tests.rs
 *
 * Dependencies (External files that this file imports):
 *   └─ serde::{Deserialize, Serialize}
 *
 * Related Documentation:
 *   └─ Spec: ./discussion.spec.md
 */
use serde::{Deserialize, Serialize};

/// Discussion（知見）の状態
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DiscussionState {
    /// Discussion 一覧
    pub discussions: Vec<Discussion>,
    /// 読み込み中フラグ
    pub loading: bool,
    /// エラーメッセージ
    pub error: Option<String>,
    /// ページネーション情報
    pub pagination: PaginationInfo,
}

/// 個別の Discussion
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Discussion {
    /// GitHub Discussion ID
    pub id: String,
    /// タイトル
    pub title: String,
    /// 本文（Markdown形式）
    pub body: String,
    /// 本文のプレビュー（最初の200文字程度）
    pub body_preview: String,
    /// 投稿者
    pub author: DiscussionAuthor,
    /// カテゴリ
    pub category: DiscussionCategory,
    /// 作成日時
    pub created_at: String,
    /// 更新日時
    pub updated_at: String,
    /// コメント数
    pub comments_count: i32,
    /// リアクション数
    pub reactions_count: i32,
    /// GitHubのURL
    pub url: String,
    /// タグ（labels）
    pub labels: Vec<String>,
}

/// Discussion の投稿者
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiscussionAuthor {
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: String,
}

/// Discussion のカテゴリ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiscussionCategory {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub emoji: Option<String>,
}

/// ページネーション情報
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PaginationInfo {
    pub has_next_page: bool,
    pub has_previous_page: bool,
    pub end_cursor: Option<String>,
    pub start_cursor: Option<String>,
    pub total_count: i32,
}
