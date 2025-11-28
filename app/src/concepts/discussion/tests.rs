// Discussion Concept - Tests
//
// DEPENDENCY MAP:
//
// Parents (Files that import this file):
//   └─ src/concepts/discussion/mod.rs
//
// Dependencies (External files that this file imports):
//   ├─ ./state.rs
//   └─ ./actions.rs
//
// Related Documentation:
//   └─ Spec: ./discussion.spec.md

use crate::concepts::discussion::actions::*;
use crate::concepts::discussion::state::*;

fn create_test_discussion(id: &str, author: &str, category_id: &str) -> Discussion {
    Discussion {
        id: id.to_string(),
        title: format!("Discussion {}", id),
        body: "Test body content".to_string(),
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

fn create_test_discussion_with_dates(
    id: &str,
    created_at: &str,
    updated_at: &str,
    comments: i32,
    reactions: i32,
) -> Discussion {
    Discussion {
        id: id.to_string(),
        title: format!("Discussion {}", id),
        body: "Test body".to_string(),
        body_preview: "Test...".to_string(),
        author: DiscussionAuthor {
            username: "test_user".to_string(),
            display_name: None,
            avatar_url: "https://example.com/avatar.png".to_string(),
        },
        category: DiscussionCategory {
            id: "cat-1".to_string(),
            name: "General".to_string(),
            description: None,
            emoji: None,
        },
        created_at: created_at.to_string(),
        updated_at: updated_at.to_string(),
        comments_count: comments,
        reactions_count: reactions,
        url: format!("https://github.com/org/repo/discussions/{}", id),
        labels: vec![],
    }
}

// TC-001: set_discussions
#[test]
fn test_set_discussions() {
    let state = DiscussionState::default();
    let discussions = vec![create_test_discussion("1", "alice", "cat-1")];

    let new_state = set_discussions(state, discussions.clone());

    assert_eq!(new_state.discussions.len(), 1);
    assert!(!new_state.loading);
    assert!(new_state.error.is_none());
}

// TC-002: append_discussions
#[test]
fn test_append_discussions() {
    let initial_discussions = vec![create_test_discussion("1", "alice", "cat-1")];
    let state = DiscussionState {
        discussions: initial_discussions,
        ..Default::default()
    };
    let new_discussions = vec![create_test_discussion("2", "bob", "cat-1")];

    let new_state = append_discussions(state, new_discussions);

    assert_eq!(new_state.discussions.len(), 2);
}

// TC-003: set_loading
#[test]
fn test_set_loading() {
    let state = DiscussionState::default();

    let loading_state = set_loading(state, true);
    assert!(loading_state.loading);

    let not_loading_state = set_loading(loading_state, false);
    assert!(!not_loading_state.loading);
}

// TC-004: set_error
#[test]
fn test_set_error() {
    let state = DiscussionState {
        loading: true,
        ..Default::default()
    };

    let error_state = set_error(state, "Something went wrong".to_string());

    assert_eq!(error_state.error, Some("Something went wrong".to_string()));
    assert!(!error_state.loading);
}

// TC-005: clear_error
#[test]
fn test_clear_error() {
    let state = DiscussionState {
        error: Some("Previous error".to_string()),
        ..Default::default()
    };

    let cleared_state = clear_error(state);

    assert!(cleared_state.error.is_none());
}

// TC-006: set_pagination
#[test]
fn test_set_pagination() {
    let state = DiscussionState::default();
    let pagination = PaginationInfo {
        has_next_page: true,
        has_previous_page: false,
        end_cursor: Some("cursor123".to_string()),
        start_cursor: None,
        total_count: 100,
    };

    let new_state = set_pagination(state, pagination.clone());

    assert_eq!(new_state.pagination, pagination);
}

// TC-007: find_discussion_by_id - found
#[test]
fn test_find_discussion_by_id_found() {
    let discussions = vec![
        create_test_discussion("1", "alice", "cat-1"),
        create_test_discussion("2", "bob", "cat-1"),
    ];

    let found = find_discussion_by_id(&discussions, "1");

    assert!(found.is_some());
    assert_eq!(found.unwrap().id, "1");
}

// TC-008: find_discussion_by_id - not found
#[test]
fn test_find_discussion_by_id_not_found() {
    let discussions = vec![create_test_discussion("1", "alice", "cat-1")];

    let found = find_discussion_by_id(&discussions, "999");

    assert!(found.is_none());
}

// TC-009: filter_by_author
#[test]
fn test_filter_by_author() {
    let discussions = vec![
        create_test_discussion("1", "alice", "cat-1"),
        create_test_discussion("2", "bob", "cat-1"),
        create_test_discussion("3", "alice", "cat-2"),
    ];

    let filtered = filter_by_author(&discussions, "alice");

    assert_eq!(filtered.len(), 2);
    assert!(filtered.iter().all(|d| d.author.username == "alice"));
}

// TC-010: filter_by_category
#[test]
fn test_filter_by_category() {
    let discussions = vec![
        create_test_discussion("1", "alice", "cat-1"),
        create_test_discussion("2", "bob", "cat-1"),
        create_test_discussion("3", "alice", "cat-2"),
    ];

    let filtered = filter_by_category(&discussions, "cat-1");

    assert_eq!(filtered.len(), 2);
    assert!(filtered.iter().all(|d| d.category.id == "cat-1"));
}

// TC-011: generate_preview - short text
#[test]
fn test_generate_preview_short_text() {
    let body = "Short text";
    let preview = generate_preview(body, 20);

    assert_eq!(preview, "Short text");
    assert!(!preview.ends_with("..."));
}

// TC-012: generate_preview - long text
#[test]
fn test_generate_preview_long_text() {
    let body =
        "This is a very long text that should be truncated because it exceeds the maximum length";
    let preview = generate_preview(body, 20);

    assert!(preview.ends_with("..."));
    assert!(preview.chars().count() <= 23); // 20 + "..."
}

// TC-013: generate_preview - markdown filtering
#[test]
fn test_generate_preview_markdown_filtering() {
    let body = "# Heading\nContent here\n```code\nfn main() {}\n```\nMore content";
    let preview = generate_preview(body, 100);

    assert!(!preview.contains("# Heading"));
    assert!(!preview.contains("```"));
    assert!(preview.contains("Content here"));
}

// TC-014: sort_by_created_at_desc
#[test]
fn test_sort_by_created_at_desc() {
    let discussions = vec![
        create_test_discussion_with_dates(
            "1",
            "2025-01-01T00:00:00Z",
            "2025-01-01T00:00:00Z",
            0,
            0,
        ),
        create_test_discussion_with_dates(
            "2",
            "2025-01-03T00:00:00Z",
            "2025-01-03T00:00:00Z",
            0,
            0,
        ),
        create_test_discussion_with_dates(
            "3",
            "2025-01-02T00:00:00Z",
            "2025-01-02T00:00:00Z",
            0,
            0,
        ),
    ];

    let sorted = sort_by_created_at_desc(discussions);

    assert_eq!(sorted[0].id, "2");
    assert_eq!(sorted[1].id, "3");
    assert_eq!(sorted[2].id, "1");
}

// TC-015: sort_by_updated_at_desc
#[test]
fn test_sort_by_updated_at_desc() {
    let discussions = vec![
        create_test_discussion_with_dates(
            "1",
            "2025-01-01T00:00:00Z",
            "2025-01-05T00:00:00Z",
            0,
            0,
        ),
        create_test_discussion_with_dates(
            "2",
            "2025-01-03T00:00:00Z",
            "2025-01-03T00:00:00Z",
            0,
            0,
        ),
        create_test_discussion_with_dates(
            "3",
            "2025-01-02T00:00:00Z",
            "2025-01-04T00:00:00Z",
            0,
            0,
        ),
    ];

    let sorted = sort_by_updated_at_desc(discussions);

    assert_eq!(sorted[0].id, "1");
    assert_eq!(sorted[1].id, "3");
    assert_eq!(sorted[2].id, "2");
}

// TC-016: sort_by_comments_desc
#[test]
fn test_sort_by_comments_desc() {
    let discussions = vec![
        create_test_discussion_with_dates(
            "1",
            "2025-01-01T00:00:00Z",
            "2025-01-01T00:00:00Z",
            5,
            0,
        ),
        create_test_discussion_with_dates(
            "2",
            "2025-01-01T00:00:00Z",
            "2025-01-01T00:00:00Z",
            20,
            0,
        ),
        create_test_discussion_with_dates(
            "3",
            "2025-01-01T00:00:00Z",
            "2025-01-01T00:00:00Z",
            10,
            0,
        ),
    ];

    let sorted = sort_by_comments_desc(discussions);

    assert_eq!(sorted[0].id, "2");
    assert_eq!(sorted[1].id, "3");
    assert_eq!(sorted[2].id, "1");
}

// TC-017: sort_by_reactions_desc
#[test]
fn test_sort_by_reactions_desc() {
    let discussions = vec![
        create_test_discussion_with_dates(
            "1",
            "2025-01-01T00:00:00Z",
            "2025-01-01T00:00:00Z",
            0,
            15,
        ),
        create_test_discussion_with_dates(
            "2",
            "2025-01-01T00:00:00Z",
            "2025-01-01T00:00:00Z",
            0,
            30,
        ),
        create_test_discussion_with_dates(
            "3",
            "2025-01-01T00:00:00Z",
            "2025-01-01T00:00:00Z",
            0,
            5,
        ),
    ];

    let sorted = sort_by_reactions_desc(discussions);

    assert_eq!(sorted[0].id, "2");
    assert_eq!(sorted[1].id, "1");
    assert_eq!(sorted[2].id, "3");
}
