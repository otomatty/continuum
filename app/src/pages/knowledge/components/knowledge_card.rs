use crate::components::{
    avatar::Avatar,
    badge::{Badge, BadgeVariant},
    card::{Card, CardBody},
};
use leptos::prelude::*;

/**
 * KnowledgeCard Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/knowledge/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   ├─ app/src/components/card.rs
 *   ├─ app/src/components/badge.rs
 *   └─ app/src/components/avatar.rs
 */
#[derive(Clone)]
pub struct KnowledgeItem {
    pub id: String,
    pub title: String,
    pub author_name: String,
    pub author_avatar: String,
    pub author_username: String,
    pub category: String,
    pub summary: String,
    pub tags: Vec<String>,
    pub reaction_count: u32,
    pub comment_count: u32,
    pub created_at: String,
}

#[component]
pub fn KnowledgeCard(
    item: KnowledgeItem,
    #[prop(optional)] on_click: Option<Callback<String>>,
) -> impl IntoView {
    let category_badge_variant = match item.category.as_str() {
        "Announcements" => BadgeVariant::Info,
        "Q&A" => BadgeVariant::Success,
        "Show and Tell" => BadgeVariant::Accent,
        "Ideas" => BadgeVariant::Warning,
        _ => BadgeVariant::Primary,
    };

    let handle_click = move |_| {
        if let Some(callback) = on_click.clone() {
            callback.run(item.id.clone());
        }
    };

    view! {
        <Card class="cursor-pointer hover:shadow-lg transition-shadow" on:click=handle_click>
            <CardBody>
                <div class="flex items-start gap-4">
                    <Avatar
                        src=item.author_avatar.clone()
                        alt=item.author_name.clone()
                        class="w-12 h-12"
                    />
                    <div class="flex-1">
                        <div class="flex items-start justify-between mb-2">
                            <div class="flex-1">
                                <h3 class="text-lg font-semibold mb-1">{item.title.clone()}</h3>
                                <div class="flex items-center gap-2 text-sm text-gray-600 mb-2">
                                    <span>{item.author_name.clone()}</span>
                                    <span>"@" {item.author_username.clone()}</span>
                                </div>
                            </div>
                            <Badge variant=category_badge_variant>{item.category.clone()}</Badge>
                        </div>
                        <p class="text-gray-700 mb-3 line-clamp-2">{item.summary.clone()}</p>
                        {if !item.tags.is_empty() {
                            Some(view! {
                                <div class="flex flex-wrap gap-2 mb-3">
                                    {item.tags.iter().map(|tag| {
                                        let tag_clone = tag.clone();
                                        view! {
                                            <span class="badge badge-outline badge-sm">{tag_clone}</span>
                                        }
                                    }).collect_view()}
                                </div>
                            })
                        } else {
                            None
                        }}
                        <div class="flex items-center justify-between text-sm text-gray-500">
                            <div class="flex items-center gap-4">
                                <span>"👍 " {item.reaction_count}</span>
                                <span>"💬 " {item.comment_count}</span>
                            </div>
                            <span>{item.created_at.clone()}</span>
                        </div>
                    </div>
                </div>
            </CardBody>
        </Card>
    }
}
