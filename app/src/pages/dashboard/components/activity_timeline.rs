use leptos::prelude::*;
use crate::components::{badge::{Badge, BadgeVariant}, avatar::Avatar};
use crate::concepts::activity::{Activity, ActivityType};

/**
 * ActivityTimeline Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/dashboard/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   ├─ app/src/components/badge.rs
 *   ├─ app/src/components/avatar.rs
 *   └─ app/src/concepts/activity/mod.rs
 */
#[component]
pub fn ActivityTimeline(
    activities: Vec<Activity>,
) -> impl IntoView {
    view! {
        <div class="space-y-4">
            {activities.into_iter().map(|activity| {
                let activity_type_badge = match activity.activity_type {
                    ActivityType::Commit => BadgeVariant::Success,
                    ActivityType::PullRequest => BadgeVariant::Primary,
                    ActivityType::Review => BadgeVariant::Info,
                    ActivityType::Issue => BadgeVariant::Warning,
                    ActivityType::Discussion => BadgeVariant::Accent,
                };
                
                let activity_type_label = match activity.activity_type {
                    ActivityType::Commit => "Commit",
                    ActivityType::PullRequest => "PR",
                    ActivityType::Review => "Review",
                    ActivityType::Issue => "Issue",
                    ActivityType::Discussion => "Discussion",
                };
                
                let display_name = activity.user.display_name.clone();
                let avatar_url = activity.user.avatar_url.clone();
                let repo_name = activity.repository.name.clone();
                let url = activity.url.clone();
                let title = activity.title.clone();
                let created_at = activity.created_at.format("%Y-%m-%d %H:%M").to_string();
                
                view! {
                    <div class="flex gap-4 p-4 border border-gray-200 rounded-lg hover:bg-gray-50">
                        <Avatar src=avatar_url alt=display_name.clone() class="w-10 h-10" />
                        <div class="flex-1">
                            <div class="flex items-center gap-2 mb-1">
                                <Badge variant=activity_type_badge>{activity_type_label}</Badge>
                                <span class="font-semibold">{display_name}</span>
                                <span class="text-gray-500 text-sm">{repo_name}</span>
                            </div>
                            <a href=url target="_blank" class="text-blue-600 hover:underline">
                                {title}
                            </a>
                            <div class="text-sm text-gray-400 mt-1">
                                {created_at}
                            </div>
                        </div>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

