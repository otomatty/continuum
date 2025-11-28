/**
 * Discussion Concept - Actions
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   ├─ src/concepts/discussion/mod.rs
 *   └─ src/concepts/discussion/tests.rs
 *
 * Dependencies (External files that this file imports):
 *   ├─ ./state.rs
 *   └─ pulldown_cmark (Markdownパーサー)
 *
 * Related Documentation:
 *   └─ Spec: ./discussion.spec.md
 */
use super::state::*;
use pulldown_cmark::{Event, Parser, Tag};

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

/// 本文プレビューを生成（最初の指定文字数）
/// pulldown-cmark を使用してMarkdownを正確にパースし、プレーンテキストを抽出
pub fn generate_preview(body: &str, max_length: usize) -> String {
    let parser = Parser::new(body);
    let mut plain_text = String::new();
    let mut in_code_block = false;

    // Markdownをパースし、テキストコンテンツのみを抽出
    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(_)) => in_code_block = true,
            Event::End(Tag::CodeBlock(_)) => in_code_block = false,
            Event::Text(text) => {
                if !in_code_block {
                    if !plain_text.is_empty() && !plain_text.ends_with(' ') {
                        plain_text.push(' ');
                    }
                    plain_text.push_str(&text);
                }
            }
            Event::SoftBreak | Event::HardBreak => {
                if !plain_text.is_empty() && !plain_text.ends_with(' ') {
                    plain_text.push(' ');
                }
            }
            _ => (),
        }
    }

    let trimmed_text = plain_text.trim();
    if trimmed_text.chars().count() <= max_length {
        trimmed_text.to_string()
    } else {
        let truncated: String = trimmed_text.chars().take(max_length).collect();
        format!("{}...", truncated)
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

/// リアクション数でソート（多い順）
pub fn sort_by_reactions_desc(mut discussions: Vec<Discussion>) -> Vec<Discussion> {
    discussions.sort_by(|a, b| b.reactions_count.cmp(&a.reactions_count));
    discussions
}
