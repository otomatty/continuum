use crate::components::{
    avatar::Avatar,
    card::{Card, CardBody, CardTitle},
    progress::Progress,
};
use crate::concepts::repository::ContributorStats;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

/**
 * TopContributorsList Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/repository/components/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   ├─ app/src/components/card.rs
 *   ├─ app/src/components/avatar.rs
 *   ├─ app/src/components/progress.rs
 *   ├─ app/src/concepts/repository/mod.rs
 *   └─ leptos_router::hooks::use_navigate
 */
#[component]
pub fn TopContributorsList(
    contributors: Vec<ContributorStats>,
    #[prop(optional)] limit: Option<usize>,
) -> impl IntoView {
    let navigate = use_navigate();
    let display_contributors: Vec<_> = if let Some(limit) = limit {
        contributors.into_iter().take(limit).collect()
    } else {
        contributors
    };

    view! {
        <Card>
            <CardTitle>
                {format!("Top Contributors{}", limit.map(|l| format!(" (Top {})", l)).unwrap_or_default())}
            </CardTitle>
            <CardBody>
                <div class="space-y-4">
                    {display_contributors.into_iter().map(|contrib| {
                        let username = contrib.user.username.clone();
                        let navigate = navigate.clone();
                        let handle_click = move |_| {
                            let username = username.clone();
                            navigate(&format!("/portfolio/{}", username), Default::default());
                        };

                        view! {
                            <div
                                class="flex items-center gap-4 p-2 rounded-lg hover:bg-base-200 cursor-pointer transition-colors"
                                on:click=handle_click
                            >
                                <Avatar
                                    src=contrib.user.avatar_url.clone()
                                    alt=contrib.user.display_name.clone()
                                    class="w-12 h-12"
                                />
                                <div class="flex-1">
                                    <div class="flex items-center justify-between mb-1">
                                        <span class="font-semibold hover:text-primary">{contrib.user.display_name.clone()}</span>
                                        <span class="text-sm text-gray-600">
                                            {format!("{:.1}%", contrib.percentage)}
                                        </span>
                                    </div>
                                    <Progress
                                        value=contrib.percentage as u32
                                        max=100
                                        class="h-2"
                                    />
                                    <div class="flex items-center justify-between mt-1 text-xs text-gray-500">
                                        <span>{format!("{} commits", contrib.commits)}</span>
                                        <span>{format!("{:.1}% of total", contrib.percentage)}</span>
                                    </div>
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </CardBody>
        </Card>
    }
}
