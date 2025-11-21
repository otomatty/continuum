use leptos::prelude::*;
use crate::components::{card::{Card, CardBody}, badge::{Badge, BadgeVariant}, avatar::Avatar};
use crate::concepts::user::{User, UserRole};

/**
 * ContributorCard Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/contributors/components/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   ├─ app/src/components/card.rs
 *   ├─ app/src/components/badge.rs
 *   ├─ app/src/components/avatar.rs
 *   └─ app/src/concepts/user/mod.rs
 */
#[component]
pub fn ContributorCard(
    user: User,
    #[prop(optional)] on_click: Option<Callback<String>>,
) -> impl IntoView {
    let role_badge_variant = match user.role {
        UserRole::CurrentEmployee => BadgeVariant::Success,
        UserRole::Alumni => BadgeVariant::Info,
        UserRole::ExternalContributor => BadgeVariant::Accent,
    };
    
    let role_label = match user.role {
        UserRole::CurrentEmployee => "Current",
        UserRole::Alumni => "Alumni",
        UserRole::ExternalContributor => "External",
    };

    let handle_click = move |_| {
        if let Some(callback) = on_click.as_ref() {
            callback.call(user.username.clone());
        }
    };

    view! {
        <Card class="cursor-pointer hover:shadow-lg transition-shadow" on:click=handle_click>
            <CardBody>
                <div class="flex flex-col items-center text-center gap-3">
                    <Avatar 
                        src=user.avatar_url.clone() 
                        alt=user.display_name.clone() 
                        class="w-20 h-20"
                    />
                    <div class="w-full">
                        <h3 class="text-lg font-semibold mb-1">{user.display_name.clone()}</h3>
                        <p class="text-sm text-gray-600 mb-2">"@" {user.username.clone()}</p>
                        <Badge variant=role_badge_variant>{role_label}</Badge>
                    </div>
                    <a 
                        href=user.github_url.clone() 
                        target="_blank" 
                        class="text-sm text-blue-600 hover:underline"
                        on:click=|ev| ev.stop_propagation()
                    >
                        "View on GitHub →"
                    </a>
                </div>
            </CardBody>
        </Card>
    }
}

