/**
 * Category Concept - State
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   ├─ src/concepts/category/mod.rs
 *   ├─ src/concepts/category/actions.rs
 *   └─ src/concepts/category/tests.rs
 *
 * Dependencies (External files that this file imports):
 *   └─ serde::{Deserialize, Serialize}
 *
 * Related Documentation:
 *   └─ Spec: ./category.spec.md
 */
use serde::{Deserialize, Serialize};

/// カテゴリの状態
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CategoryState {
    /// カテゴリ一覧
    pub categories: Vec<Category>,
    /// 選択中のカテゴリID
    pub selected_category_id: Option<String>,
    /// 読み込み中フラグ
    pub loading: bool,
    /// エラーメッセージ
    pub error: Option<String>,
}

/// カテゴリ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub emoji: Option<String>,
    /// このカテゴリのDiscussion数
    pub discussions_count: i32,
}

impl Category {
    /// 表示名（絵文字 + 名前）
    pub fn display_name(&self) -> String {
        match &self.emoji {
            Some(emoji) => format!("{} {}", emoji, self.name),
            None => self.name.clone(),
        }
    }
}
