use crate::components::{
    avatar::Avatar,
    badge::{Badge, BadgeVariant},
    card::{Card, CardBody},
};
use crate::concepts::user::{User, UserRole};
use leptos::prelude::*;

/**
 * UserProfile Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/portfolio/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   ├─ app/src/components/card.rs
 *   ├─ app/src/components/badge.rs
 *   ├─ app/src/components/avatar.rs
 *   └─ app/src/concepts/user/mod.rs
 */
#[component]
pub fn UserProfile(user: User) -> impl IntoView {
    let role_badge_variant = match user.role {
        UserRole::CurrentEmployee => BadgeVariant::Success,
        UserRole::Alumni => BadgeVariant::Info,
        UserRole::ExternalContributor => BadgeVariant::Accent,
    };

    let role_label = match user.role {
        UserRole::CurrentEmployee => "Current Employee",
        UserRole::Alumni => "Alumni",
        UserRole::ExternalContributor => "External Contributor",
    };

    view! {
        <Card>
            <CardBody>
                <div class="flex flex-col md:flex-row gap-6">
                    <Avatar src=user.avatar_url.clone() alt=user.display_name.clone() class="w-24 h-24" />
                    <div class="flex-1">
                        <div class="flex items-center gap-3 mb-2">
                            <h2 class="text-3xl font-bold">{user.display_name.clone()}</h2>
                            <Badge variant=role_badge_variant>{role_label}</Badge>
                        </div>
                        <p class="text-gray-600 mb-4">"@" {user.username.clone()}</p>
                        <div class="flex flex-wrap gap-4 text-sm">
                            {user.joined_at.map(|joined_at| view! {
                                <div>
                                    <span class="text-gray-500">"Joined: "</span>
                                    <span class="font-semibold">{joined_at.format("%Y-%m-%d").to_string()}</span>
                                </div>
                            })}
                            {user.left_at.map(|left_at| view! {
                                <div>
                                    <span class="text-gray-500">"Left: "</span>
                                    <span class="font-semibold">{left_at.format("%Y-%m-%d").to_string()}</span>
                                </div>
                            })}
                            <a href=user.github_url.clone() target="_blank" class="text-blue-600 hover:underline">
                                "View on GitHub →"
                            </a>
                        </div>
                    </div>
                </div>
            </CardBody>
        </Card>
    }
}
