pub mod components;

use crate::components::breadcrumbs::{BreadcrumbItem, Breadcrumbs};
use crate::components::{
    avatar::Avatar,
    badge::{Badge, BadgeVariant},
    button::{Button, ButtonVariant},
};
use crate::pages::knowledge::components::KnowledgeItem;
use components::{
    Comment, CommentSection, MarkdownRenderer, Reaction, ReactionButtons, RelatedArticles,
};
use leptos::prelude::*;

/**
 * KnowledgeDetailPage Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/lib.rs
 *
 * Dependencies (External files that this component imports):
 *   ├─ app/src/pages/knowledge_detail/components/mod.rs
 *   ├─ app/src/components/breadcrumbs.rs
 *   ├─ app/src/components/badge.rs
 *   ├─ app/src/components/avatar.rs
 *   └─ app/src/components/button.rs
 */
#[component]
pub fn KnowledgeDetailPage() -> impl IntoView {
    // Mock data - will be replaced with API calls later
    let mock_article = KnowledgeItem {
        id: "1".to_string(),
        title: "Rustのベストプラクティス".to_string(),
        author_name: "Alice Developer".to_string(),
        author_avatar: "https://api.dicebear.com/7.x/avataaars/svg?seed=Alice".to_string(),
        author_username: "alice-dev".to_string(),
        category: "Show and Tell".to_string(),
        summary: "Rustでプロジェクトを開発する際のベストプラクティスをまとめました。".to_string(),
        tags: vec!["rust".to_string(), "best-practices".to_string()],
        reaction_count: 15,
        comment_count: 8,
        created_at: "2025-11-20".to_string(),
    };

    let mock_content = r#"
# Rustのベストプラクティス

Rustでプロジェクトを開発する際のベストプラクティスをまとめました。

## 所有権とライフタイム

Rustの所有権システムは、メモリ安全性を保証するための重要な機能です。

\`\`\`rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // エラー: sは既に移動されている
}
\`\`\`

## エラーハンドリング

Rustでは、`Result<T, E>`型を使ってエラーハンドリングを行います。

\`\`\`rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}
\`\`\`
"#;

    let mock_reactions = vec![
        Reaction {
            emoji: "👍".to_string(),
            count: 15,
            active: false,
        },
        Reaction {
            emoji: "❤️".to_string(),
            count: 8,
            active: true,
        },
        Reaction {
            emoji: "🎉".to_string(),
            count: 5,
            active: false,
        },
        Reaction {
            emoji: "🚀".to_string(),
            count: 3,
            active: false,
        },
    ];

    let mock_comments = vec![
        Comment {
            id: "1".to_string(),
            author_name: "Bob Contributor".to_string(),
            author_avatar: "https://api.dicebear.com/7.x/avataaars/svg?seed=Bob".to_string(),
            author_username: "bob-contrib".to_string(),
            content: "<p>とても参考になりました！特に所有権の部分が理解しやすかったです。</p>".to_string(),
            created_at: "2025-11-20 10:30".to_string(),
            reactions: vec![("👍".to_string(), 3)],
            replies: vec![],
        },
        Comment {
            id: "2".to_string(),
            author_name: "Charlie Maintainer".to_string(),
            author_avatar: "https://api.dicebear.com/7.x/avataaars/svg?seed=Charlie".to_string(),
            author_username: "charlie-maintainer".to_string(),
            content: "<p>エラーハンドリングの部分について、もう少し詳しく説明していただけますか？</p>".to_string(),
            created_at: "2025-11-20 14:15".to_string(),
            reactions: vec![],
            replies: vec![
                Comment {
                    id: "2-1".to_string(),
                    author_name: "Alice Developer".to_string(),
                    author_avatar: "https://api.dicebear.com/7.x/avataaars/svg?seed=Alice".to_string(),
                    author_username: "alice-dev".to_string(),
                    content: "<p>もちろんです！`Result`型の使い方について、別の記事で詳しく説明しますね。</p>".to_string(),
                    created_at: "2025-11-20 15:00".to_string(),
                    reactions: vec![("👍".to_string(), 2)],
                    replies: vec![],
                },
            ],
        },
    ];

    let mock_related = vec![KnowledgeItem {
        id: "2".to_string(),
        title: "GitHub ActionsのCI/CD設定".to_string(),
        author_name: "Bob Contributor".to_string(),
        author_avatar: "https://api.dicebear.com/7.x/avataaars/svg?seed=Bob".to_string(),
        author_username: "bob-contrib".to_string(),
        category: "Q&A".to_string(),
        summary: "GitHub Actionsを使ったCI/CDパイプラインの設定方法について質問があります。"
            .to_string(),
        tags: vec!["github-actions".to_string(), "ci-cd".to_string()],
        reaction_count: 23,
        comment_count: 12,
        created_at: "2025-11-19".to_string(),
    }];

    let category_badge_variant = match mock_article.category.as_str() {
        "Announcements" => BadgeVariant::Info,
        "Q&A" => BadgeVariant::Success,
        "Show and Tell" => BadgeVariant::Accent,
        "Ideas" => BadgeVariant::Warning,
        _ => BadgeVariant::Primary,
    };

    // Clone values needed in the view to avoid move issues
    // Create separate clones for values used multiple times
    let article_category_breadcrumb = mock_article.category.clone();
    let article_category_badge = mock_article.category.clone();
    let article_title_breadcrumb = mock_article.title.clone();
    let article_title_header = mock_article.title.clone();
    let article_tags_clone = mock_article.tags.clone();
    let article_author_avatar_clone = mock_article.author_avatar.clone();
    let article_author_name_avatar = mock_article.author_name.clone();
    let article_author_name_text = mock_article.author_name.clone();
    let article_author_username_clone = mock_article.author_username.clone();
    let article_created_at_clone = mock_article.created_at.clone();

    let handle_reaction = move |_emoji: String| {
        // Handle reaction click
    };

    let handle_reply = move |_comment_id: String| {
        // Handle reply click
    };

    let handle_related_click = move |_id: String| {
        // Handle related article click
    };

    view! {
        <div class="container mx-auto px-4 py-8">
            <Breadcrumbs>
                <BreadcrumbItem href="/knowledge".to_string()>"Knowledge"</BreadcrumbItem>
                <BreadcrumbItem>{article_category_breadcrumb}</BreadcrumbItem>
                <BreadcrumbItem>{article_title_breadcrumb}</BreadcrumbItem>
            </Breadcrumbs>

            <article class="mt-8">
                <header class="mb-8">
                    <h1 class="text-4xl font-bold mb-4">{article_title_header}</h1>
                    <div class="flex items-center gap-4 mb-4">
                        <Avatar
                            src=article_author_avatar_clone
                            alt=article_author_name_avatar
                            class="w-12 h-12"
                        />
                        <div>
                            <div class="font-semibold">{article_author_name_text}</div>
                            <div class="text-sm text-gray-600">"@" {article_author_username_clone}</div>
                        </div>
                        <span class="text-gray-400">{article_created_at_clone}</span>
                    </div>
                    <div class="flex items-center gap-4 mb-4">
                        <Badge variant=category_badge_variant>{article_category_badge}</Badge>
                        {article_tags_clone.into_iter().map(|tag| {
                            view! {
                                <span class="badge badge-outline">{tag}</span>
                            }
                        }).collect_view()}
                    </div>
                    <div class="mb-4">
                        <ReactionButtons
                            reactions=mock_reactions.clone()
                            on_reaction=Callback::new(handle_reaction)
                        />
                    </div>
                    <Button
                        variant=ButtonVariant::Ghost
                        class="text-sm".to_string()
                        on_click=Callback::new(move |_| {
                            // Open in GitHub
                        })
                    >
                        "Open in GitHub →"
                    </Button>
                </header>

                <div class="mb-8">
                    <MarkdownRenderer content=mock_content.to_string() />
                </div>

                <div class="mb-8">
                    <CommentSection
                        comments=mock_comments.clone()
                        on_reply=Callback::new(handle_reply)
                    />
                </div>

                <div>
                    <RelatedArticles
                        articles=mock_related.clone()
                        on_click=Callback::new(handle_related_click)
                    />
                </div>
            </article>
        </div>
    }
}
