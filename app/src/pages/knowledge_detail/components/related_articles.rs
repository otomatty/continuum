use crate::pages::knowledge::components::knowledge_card::{KnowledgeCard, KnowledgeItem};
use leptos::prelude::*;

/**
 * RelatedArticles Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/knowledge_detail/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   └─ app/src/pages/knowledge/components/knowledge_card.rs
 */
#[component]
pub fn RelatedArticles(
    articles: Vec<KnowledgeItem>,
    #[prop(optional)] on_click: Option<Callback<String>>,
) -> impl IntoView {
    view! {
        <div class="space-y-4">
            <h3 class="text-2xl font-bold mb-4">"Related Articles"</h3>
            {if articles.is_empty() {
                view! {
                    <p class="text-gray-500">"No related articles found."</p>
                }.into_any()
            } else {
                view! {
                    <div class="space-y-4">
                        {articles.into_iter().map(|article| {
                            let on_click_clone = on_click.clone();
                            if let Some(callback) = on_click_clone {
                                view! {
                                    <KnowledgeCard
                                        item=article
                                        on_click=callback
                                    />
                                }
                            } else {
                                view! {
                                    <KnowledgeCard
                                        item=article
                                    />
                                }
                            }
                        }).collect_view()}
                    </div>
                }.into_any()
            }}
        </div>
    }
}
