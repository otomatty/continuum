use crate::components::{
    badge::{Badge, BadgeVariant},
    card::{Card, CardBody},
};
use crate::concepts::repository::Repository;
use leptos::prelude::*;

/**
 * RepositoryHeader Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/repository/components/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   ├─ app/src/components/card.rs
 *   ├─ app/src/components/badge.rs
 *   └─ app/src/concepts/repository/mod.rs
 */
#[component]
pub fn RepositoryHeader(repository: Repository) -> impl IntoView {
    view! {
        <Card>
            <CardBody>
                <div class="space-y-4">
                    <div class="flex items-start justify-between">
                        <div class="flex-1">
                            <h1 class="text-3xl font-bold mb-2">
                                <a
                                    href=format!("https://github.com/{}", repository.full_name.clone())
                                    target="_blank"
                                    class="hover:underline"
                                >
                                    {repository.name.clone()}
                                </a>
                            </h1>
                            {repository.description.clone().map(|desc| view! {
                                <p class="text-gray-600 mb-4">{desc}</p>
                            })}
                        </div>
                        <div class="flex items-center gap-1 text-xl">
                            <span class="text-yellow-500">{"★"}</span>
                            <span class="font-bold">{repository.stars}</span>
                        </div>
                    </div>

                    <div class="flex flex-wrap items-center gap-4 text-sm">
                        {repository.language.clone().map(|lang| view! {
                            <Badge variant=BadgeVariant::Primary>{lang}</Badge>
                        })}
                        <div class="text-gray-600">
                            <span class="font-medium">{repository.contributors.len()}</span>
                            <span>" contributors"</span>
                        </div>
                        <div class="text-gray-600">
                            <span>"Last updated: "</span>
                            <span class="font-medium">{repository.updated_at.format("%Y-%m-%d").to_string()}</span>
                        </div>
                        <a
                            href=format!("https://github.com/{}", repository.full_name.clone())
                            target="_blank"
                            class="text-blue-600 hover:underline"
                        >
                            "View on GitHub →"
                        </a>
                    </div>
                </div>
            </CardBody>
        </Card>
    }
}
