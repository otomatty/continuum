# Task 10: Discussion Concepts 実装

## 1. 目的と背景

### なぜこのタスクが必要か
PRDの「5.2 知見共有機能」に記載されているGitHub Discussionsをデータソースとしたナレッジフィード機能を実装するために、Discussion関連のConceptを定義します。

### 完成時のユーザー体験
- GitHub Discussionsの投稿がブログ形式で表示される
- カテゴリや投稿者による検索・フィルタリングができる
- Discussion詳細ページでコンテンツを閲覧できる

---

## 2. 前提条件

### 依存するタスク
- ✅ Task 5: GitHub API 実装
- ✅ Task 6: Search & Filter Concepts

### 必要な知識
- Legible Architecture の Concept パターン
- GitHub Discussions の構造

---

## 3. 作成ファイル一覧

### Discussion Concept
| ファイル | 内容 |
|---------|------|
| `app/src/concepts/discussion/state.rs` | Discussion 状態の型定義 |
| `app/src/concepts/discussion/actions.rs` | Discussion 操作ロジック |
| `app/src/concepts/discussion/discussion.spec.md` | 仕様書 |
| `app/src/concepts/discussion/tests.rs` | テスト |
| `app/src/concepts/discussion/mod.rs` | モジュール定義 |

### Category Concept
| ファイル | 内容 |
|---------|------|
| `app/src/concepts/category/state.rs` | カテゴリ状態の型定義 |
| `app/src/concepts/category/actions.rs` | カテゴリ操作ロジック |
| `app/src/concepts/category/category.spec.md` | 仕様書 |
| `app/src/concepts/category/tests.rs` | テスト |
| `app/src/concepts/category/mod.rs` | モジュール定義 |

---

## 4. 実装手順

### Step 1: Discussion Concept の状態定義

`app/src/concepts/discussion/state.rs`:

```rust
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
```

### Step 2: Discussion Concept のアクション定義

`app/src/concepts/discussion/actions.rs`:

```rust
use super::state::*;

/// Discussion 一覧を設定
pub fn set_discussions(state: DiscussionState, discussions: Vec<Discussion>) -> DiscussionState {
    DiscussionState {
        discussions,
        loading: false,
        error: None,
        ..state
    }
}

/// Discussion を追加（ページネーション用）
pub fn append_discussions(
    state: DiscussionState,
    new_discussions: Vec<Discussion>,
) -> DiscussionState {
    let mut discussions = state.discussions;
    discussions.extend(new_discussions);
    DiscussionState {
        discussions,
        ..state
    }
}

/// 読み込み開始
pub fn set_loading(state: DiscussionState, loading: bool) -> DiscussionState {
    DiscussionState { loading, ..state }
}

/// エラーを設定
pub fn set_error(state: DiscussionState, error: String) -> DiscussionState {
    DiscussionState {
        error: Some(error),
        loading: false,
        ..state
    }
}

/// エラーをクリア
pub fn clear_error(state: DiscussionState) -> DiscussionState {
    DiscussionState {
        error: None,
        ..state
    }
}

/// ページネーション情報を更新
pub fn set_pagination(state: DiscussionState, pagination: PaginationInfo) -> DiscussionState {
    DiscussionState {
        pagination,
        ..state
    }
}

/// IDでDiscussionを検索
pub fn find_discussion_by_id<'a>(
    discussions: &'a [Discussion],
    id: &str,
) -> Option<&'a Discussion> {
    discussions.iter().find(|d| d.id == id)
}

/// 投稿者でフィルタリング
pub fn filter_by_author(discussions: &[Discussion], username: &str) -> Vec<Discussion> {
    discussions
        .iter()
        .filter(|d| d.author.username == username)
        .cloned()
        .collect()
}

/// カテゴリでフィルタリング
pub fn filter_by_category(discussions: &[Discussion], category_id: &str) -> Vec<Discussion> {
    discussions
        .iter()
        .filter(|d| d.category.id == category_id)
        .cloned()
        .collect()
}

/// 本文プレビューを生成（最初の200文字）
pub fn generate_preview(body: &str, max_length: usize) -> String {
    // Markdownの装飾を簡易的に除去
    let plain_text: String = body
        .lines()
        .filter(|line| !line.starts_with('#') && !line.starts_with("```"))
        .collect::<Vec<_>>()
        .join(" ");

    if plain_text.len() <= max_length {
        plain_text
    } else {
        format!("{}...", &plain_text[..max_length])
    }
}

/// 作成日時でソート（新しい順）
pub fn sort_by_created_at_desc(mut discussions: Vec<Discussion>) -> Vec<Discussion> {
    discussions.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    discussions
}

/// 更新日時でソート（新しい順）
pub fn sort_by_updated_at_desc(mut discussions: Vec<Discussion>) -> Vec<Discussion> {
    discussions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    discussions
}

/// コメント数でソート（多い順）
pub fn sort_by_comments_desc(mut discussions: Vec<Discussion>) -> Vec<Discussion> {
    discussions.sort_by(|a, b| b.comments_count.cmp(&a.comments_count));
    discussions
}
```

### Step 3: Category Concept の状態定義

`app/src/concepts/category/state.rs`:

```rust
use serde::{Deserialize, Serialize};

/// カテゴリの状態
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CategoryState {
    /// カテゴリ一覧
    pub categories: Vec<Category>,
    /// 選択中のカテゴリID
    pub selected_category_id: Option<String>,
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
```

### Step 4: Category Concept のアクション定義

`app/src/concepts/category/actions.rs`:

```rust
use super::state::*;

/// カテゴリ一覧を設定
pub fn set_categories(state: CategoryState, categories: Vec<Category>) -> CategoryState {
    CategoryState { categories, ..state }
}

/// カテゴリを選択
pub fn select_category(state: CategoryState, category_id: Option<String>) -> CategoryState {
    CategoryState {
        selected_category_id: category_id,
        ..state
    }
}

/// IDでカテゴリを検索
pub fn find_category_by_id<'a>(categories: &'a [Category], id: &str) -> Option<&'a Category> {
    categories.iter().find(|c| c.id == id)
}

/// 名前でカテゴリを検索
pub fn find_category_by_name<'a>(categories: &'a [Category], name: &str) -> Option<&'a Category> {
    categories.iter().find(|c| c.name == name)
}

/// Discussion数でソート（多い順）
pub fn sort_by_discussions_count_desc(mut categories: Vec<Category>) -> Vec<Category> {
    categories.sort_by(|a, b| b.discussions_count.cmp(&a.discussions_count));
    categories
}

/// 名前でソート
pub fn sort_by_name(mut categories: Vec<Category>) -> Vec<Category> {
    categories.sort_by(|a, b| a.name.cmp(&b.name));
    categories
}
```

### Step 5: テストの作成

`app/src/concepts/discussion/tests.rs`:

```rust
#[cfg(test)]
mod tests {
    use crate::concepts::discussion::actions::*;
    use crate::concepts::discussion::state::*;

    fn create_test_discussion(id: &str, author: &str, category_id: &str) -> Discussion {
        Discussion {
            id: id.to_string(),
            title: format!("Discussion {}", id),
            body: "Test body".to_string(),
            body_preview: "Test...".to_string(),
            author: DiscussionAuthor {
                username: author.to_string(),
                display_name: None,
                avatar_url: "https://example.com/avatar.png".to_string(),
            },
            category: DiscussionCategory {
                id: category_id.to_string(),
                name: "General".to_string(),
                description: None,
                emoji: Some("💬".to_string()),
            },
            created_at: "2025-01-01T00:00:00Z".to_string(),
            updated_at: "2025-01-01T00:00:00Z".to_string(),
            comments_count: 5,
            reactions_count: 10,
            url: "https://github.com/org/repo/discussions/1".to_string(),
            labels: vec![],
        }
    }

    #[test]
    fn test_set_discussions() {
        let state = DiscussionState::default();
        let discussions = vec![create_test_discussion("1", "alice", "cat-1")];
        
        let new_state = set_discussions(state, discussions.clone());
        
        assert_eq!(new_state.discussions.len(), 1);
        assert!(!new_state.loading);
    }

    #[test]
    fn test_filter_by_author() {
        let discussions = vec![
            create_test_discussion("1", "alice", "cat-1"),
            create_test_discussion("2", "bob", "cat-1"),
            create_test_discussion("3", "alice", "cat-2"),
        ];
        
        let filtered = filter_by_author(&discussions, "alice");
        
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_filter_by_category() {
        let discussions = vec![
            create_test_discussion("1", "alice", "cat-1"),
            create_test_discussion("2", "bob", "cat-1"),
            create_test_discussion("3", "alice", "cat-2"),
        ];
        
        let filtered = filter_by_category(&discussions, "cat-1");
        
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_generate_preview() {
        let body = "This is a very long text that should be truncated";
        let preview = generate_preview(body, 20);
        
        assert!(preview.ends_with("..."));
        assert!(preview.len() <= 23); // 20 + "..."
    }

    #[test]
    fn test_find_discussion_by_id() {
        let discussions = vec![
            create_test_discussion("1", "alice", "cat-1"),
            create_test_discussion("2", "bob", "cat-1"),
        ];
        
        let found = find_discussion_by_id(&discussions, "1");
        
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, "1");
    }
}
```

### Step 6: mod.rs の作成

`app/src/concepts/discussion/mod.rs`:

```rust
/**
 * Discussion Concept
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this Concept):
 *   ├─ app/src/concepts/mod.rs
 *   ├─ app/src/pages/knowledge/mod.rs
 *   └─ app/src/pages/knowledge_detail/mod.rs
 *
 * Related Documentation:
 *   └─ Spec: ./discussion.spec.md
 */

pub mod actions;
pub mod state;

#[cfg(test)]
mod tests;

pub use actions::*;
pub use state::*;
```

`app/src/concepts/category/mod.rs`:

```rust
/**
 * Category Concept
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this Concept):
 *   ├─ app/src/concepts/mod.rs
 *   └─ app/src/pages/knowledge/mod.rs
 *
 * Related Documentation:
 *   └─ Spec: ./category.spec.md
 */

pub mod actions;
pub mod state;

#[cfg(test)]
mod tests;

pub use actions::*;
pub use state::*;
```

### Step 7: concepts/mod.rs の更新

```rust
pub mod discussion;
pub mod category;
```

---

## 5. テスト実行

```bash
# Discussion Concept のテスト
cargo test -p app discussion::

# Category Concept のテスト
cargo test -p app category::

# 全テスト
bun run test:app
```

---

## 6. 完了条件チェックリスト

- [ ] Discussion Concept が実装されている
  - [ ] state.rs（Discussion, DiscussionAuthor, DiscussionCategory, PaginationInfo）
  - [ ] actions.rs（set_discussions, filter_by_*, sort_by_* など）
  - [ ] tests.rs
  - [ ] mod.rs
  - [ ] discussion.spec.md
- [ ] Category Concept が実装されている
  - [ ] state.rs
  - [ ] actions.rs
  - [ ] tests.rs
  - [ ] mod.rs
  - [ ] category.spec.md
- [ ] concepts/mod.rs に両モジュールが追加されている
- [ ] 全テストが通る
- [ ] ビルドエラーがない

---

## 7. 参照ドキュメント

- PRD: `PRD.md` - セクション 5.2
- Legible Architecture: ワークスペースの `.cursor/rules` 参照
- 既存 Concept 実装例: `app/src/concepts/user/`

---

## 8. 注意点

- **Concept の独立性**: Discussion と Category は互いに直接参照しない
- **純粋関数**: すべての actions は副作用を含まない
- **型安全性**: Discussion と Category の関連は ID 参照で表現

