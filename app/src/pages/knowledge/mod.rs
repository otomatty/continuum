pub mod components;

use components::{
    AuthorFilter, CategoryFilter, KnowledgeCard, KnowledgeCategory, KnowledgeItem, SearchBar,
};
use leptos::prelude::*;

/**
 * KnowledgePage Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/lib.rs
 *
 * Dependencies (External files that this component imports):
 *   └─ app/src/pages/knowledge/components/mod.rs
 */
#[component]
pub fn KnowledgePage() -> impl IntoView {
    // Mock data - will be replaced with API calls later
    let mock_knowledge_items = vec![
        KnowledgeItem {
            id: "1".to_string(),
            title: "Rustのベストプラクティス".to_string(),
            author_name: "Alice Developer".to_string(),
            author_avatar: "https://api.dicebear.com/7.x/avataaars/svg?seed=Alice".to_string(),
            author_username: "alice-dev".to_string(),
            category: "Show and Tell".to_string(),
            summary: "Rustでプロジェクトを開発する際のベストプラクティスをまとめました。所有権、ライフタイム、エラーハンドリングなどについて解説しています。".to_string(),
            tags: vec!["rust".to_string(), "best-practices".to_string()],
            reaction_count: 15,
            comment_count: 8,
            created_at: "2025-11-20".to_string(),
        },
        KnowledgeItem {
            id: "2".to_string(),
            title: "GitHub ActionsのCI/CD設定".to_string(),
            author_name: "Bob Contributor".to_string(),
            author_avatar: "https://api.dicebear.com/7.x/avataaars/svg?seed=Bob".to_string(),
            author_username: "bob-contrib".to_string(),
            category: "Q&A".to_string(),
            summary: "GitHub Actionsを使ったCI/CDパイプラインの設定方法について質問があります。".to_string(),
            tags: vec!["github-actions".to_string(), "ci-cd".to_string()],
            reaction_count: 23,
            comment_count: 12,
            created_at: "2025-11-19".to_string(),
        },
        KnowledgeItem {
            id: "3".to_string(),
            title: "新機能リリースのお知らせ".to_string(),
            author_name: "Charlie Maintainer".to_string(),
            author_avatar: "https://api.dicebear.com/7.x/avataaars/svg?seed=Charlie".to_string(),
            author_username: "charlie-maintainer".to_string(),
            category: "Announcements".to_string(),
            summary: "Continuum v2.0の新機能についてお知らせします。".to_string(),
            tags: vec!["announcement".to_string(), "release".to_string()],
            reaction_count: 45,
            comment_count: 20,
            created_at: "2025-11-18".to_string(),
        },
    ];

    let mock_knowledge_items_clone = mock_knowledge_items.clone();
    let (filtered_items, set_filtered_items) = signal(mock_knowledge_items.clone());
    let (search_query, set_search_query) = signal(String::new());
    let (selected_category, set_selected_category) = signal(KnowledgeCategory::All);
    let (selected_author, set_selected_author) = signal(String::new());

    let handle_search = {
        let mock_knowledge_items_clone = mock_knowledge_items_clone.clone();
        move |query: String| {
            set_search_query.set(query.clone());
            filter_items(
                query,
                selected_category.get(),
                selected_author.get(),
                &mock_knowledge_items_clone,
                &set_filtered_items,
            );
        }
    };

    let handle_category_change = {
        let mock_knowledge_items_clone = mock_knowledge_items_clone.clone();
        move |category: KnowledgeCategory| {
            set_selected_category.set(category);
            filter_items(
                search_query.get(),
                category,
                selected_author.get(),
                &mock_knowledge_items_clone,
                &set_filtered_items,
            );
        }
    };

    let handle_author_change = {
        let mock_knowledge_items_clone = mock_knowledge_items_clone.clone();
        move |author: String| {
            set_selected_author.set(author.clone());
            filter_items(
                search_query.get(),
                selected_category.get(),
                author,
                &mock_knowledge_items_clone,
                &set_filtered_items,
            );
        }
    };

    let handle_card_click = move |_id: String| {
        // Navigate to knowledge detail page
        // This will be implemented when routing is set up
    };

    let mock_authors = vec![
        "alice-dev".to_string(),
        "bob-contrib".to_string(),
        "charlie-maintainer".to_string(),
    ];

    view! {
        <div class="container mx-auto px-4 py-8">
            <div class="mb-8">
                <h1 class="text-4xl font-bold mb-2">"Knowledge Base"</h1>
                <p class="text-gray-600">"組織の知見を共有・発見する場所"</p>
            </div>

            <div class="mb-6 space-y-4 md:flex md:space-y-0 md:space-x-4">
                <div class="flex-1">
                    <SearchBar
                        placeholder="検索...".to_string()
                        on_search=Callback::new(handle_search)
                    />
                </div>
                <CategoryFilter on_change=Callback::new(handle_category_change) />
                <AuthorFilter
                    authors=mock_authors
                    on_change=Callback::new(handle_author_change)
                />
            </div>

            <div class="space-y-4">
                {move || filtered_items.get().iter().map(|item| {
                    view! {
                    <KnowledgeCard
                        item=item.clone()
                        on_click=Callback::new(handle_card_click.clone())
                    />
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

fn filter_items(
    search_query: String,
    category: KnowledgeCategory,
    author: String,
    items: &[KnowledgeItem],
    set_filtered: &WriteSignal<Vec<KnowledgeItem>>,
) {
    let filtered: Vec<KnowledgeItem> = items
        .iter()
        .filter(|item| {
            let matches_search = search_query.is_empty()
                || item
                    .title
                    .to_lowercase()
                    .contains(&search_query.to_lowercase())
                || item
                    .summary
                    .to_lowercase()
                    .contains(&search_query.to_lowercase());

            let matches_category =
                category == KnowledgeCategory::All || item.category == category.as_str();

            let matches_author = author.is_empty() || item.author_username == author;

            matches_search && matches_category && matches_author
        })
        .cloned()
        .collect();

    set_filtered.set(filtered);
}
