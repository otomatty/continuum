use crate::components::{
    avatar::Avatar,
    badge::{Badge, BadgeVariant},
};
use crate::concepts::activity::{Activity, ActivityType};
use leptos::prelude::*;

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
 *
 * Note: Activity Concept は ID 参照のみ保持するため、
 * 表示に必要な情報は ActivityDisplay として UI 層で結合する
 */

/// UI 表示用の Activity データ（Synchronization/UI 層で結合）
#[derive(Debug, Clone)]
pub struct ActivityDisplay {
    pub activity: Activity,
    pub user_display_name: String,
    pub user_avatar_url: String,
    pub repository_name: String,
}

impl ActivityDisplay {
    /// Activity と関連データから表示用データを生成
    pub fn from_activity(
        activity: Activity,
        user_display_name: String,
        user_avatar_url: String,
        repository_name: String,
    ) -> Self {
        Self {
            activity,
            user_display_name,
            user_avatar_url,
            repository_name,
        }
    }

    /// モックデータから表示用データを生成（開発用）
    pub fn from_activity_with_mock(activity: Activity) -> Self {
        // ID からモック表示名を生成
        let user_display_name = format!("User {}", activity.user_id);
        let user_avatar_url = format!(
            "https://avatars.githubusercontent.com/u/{}",
            activity.user_id.chars().take(8).collect::<String>()
        );
        let repository_name = activity.repository_id.clone();

        Self {
            activity,
            user_display_name,
            user_avatar_url,
            repository_name,
        }
    }
}

#[component]
pub fn ActivityTimeline(activities: Vec<ActivityDisplay>) -> impl IntoView {
    view! {
        <div class="space-y-4">
            {activities.into_iter().map(|activity_display| {
                let activity_type_badge = match activity_display.activity.activity_type {
                    ActivityType::Commit => BadgeVariant::Success,
                    ActivityType::PullRequest => BadgeVariant::Primary,
                    ActivityType::Review => BadgeVariant::Info,
                    ActivityType::Issue => BadgeVariant::Warning,
                    ActivityType::Discussion => BadgeVariant::Accent,
                };

                let activity_type_label = match activity_display.activity.activity_type {
                    ActivityType::Commit => "Commit",
                    ActivityType::PullRequest => "PR",
                    ActivityType::Review => "Review",
                    ActivityType::Issue => "Issue",
                    ActivityType::Discussion => "Discussion",
                };

                let display_name = activity_display.user_display_name.clone();
                let avatar_url = activity_display.user_avatar_url.clone();
                let repo_name = activity_display.repository_name.clone();
                let url = activity_display.activity.url.clone();
                let title = activity_display.activity.title.clone();
                let created_at = activity_display.activity.created_at.format("%Y-%m-%d %H:%M").to_string();

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
