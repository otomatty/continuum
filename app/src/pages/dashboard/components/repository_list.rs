use leptos::prelude::*;
use crate::components::{card::{Card, CardBody}, badge::{Badge, BadgeVariant}, avatar::Avatar};
use crate::concepts::repository::Repository;

/**
 * RepositoryList Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/dashboard/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   ├─ app/src/components/card.rs
 *   ├─ app/src/components/badge.rs
 *   ├─ app/src/components/avatar.rs
 *   └─ app/src/concepts/repository/mod.rs
 */
#[component]
pub fn RepositoryList(
    repositories: Vec<Repository>,
) -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            {repositories.into_iter().map(|repo| {
                view! {
                    <Card>
                        <CardBody>
                            <div class="flex justify-between items-start mb-2">
                                <div>
                                    <h3 class="text-lg font-semibold">
                                        <a href=format!("https://github.com/{}", repo.full_name) target="_blank" class="hover:underline">
                                            {repo.name.clone()}
                                        </a>
                                    </h3>
                                    {repo.description.clone().map(|desc| view! { <p class="text-sm text-gray-600 mt-1">{desc}</p> })}
                                </div>
                                <div class="flex items-center gap-1">
                                    <span class="text-yellow-500">{"★"}</span>
                                    <span class="font-semibold">{repo.stars}</span>
                                </div>
                            </div>
                            <div class="flex items-center gap-4 mt-4 text-sm">
                                {repo.language.clone().map(|lang| view! {
                                    <Badge variant=BadgeVariant::Ghost>{lang}</Badge>
                                })}
                                <span class="text-gray-500">
                                    {"Updated: "} {repo.updated_at.format("%Y-%m-%d").to_string()}
                                </span>
                            </div>
                            {if !repo.contributors.is_empty() {
                                Some(view! {
                                    <div class="mt-3">
                                        <p class="text-xs text-gray-500 mb-2">Top Contributors:</p>
                                        <div class="flex gap-2">
                                            {repo.contributors.iter().take(3).map(|contrib| {
                                                view! {
                                                    <Avatar src=contrib.user.avatar_url.clone() alt=contrib.user.display_name.clone() class="w-6 h-6" />
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    </div>
                                })
                            } else {
                                None
                            }}
                        </CardBody>
                    </Card>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

