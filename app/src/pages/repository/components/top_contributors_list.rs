use leptos::prelude::*;
use crate::components::{
    card::{Card, CardTitle, CardBody},
    avatar::Avatar,
    progress::Progress,
};
use crate::concepts::repository::ContributorStats;

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
 *   └─ app/src/concepts/repository/mod.rs
 */
#[component]
pub fn TopContributorsList(
    contributors: Vec<ContributorStats>,
    #[prop(optional)] limit: Option<usize>,
) -> impl IntoView {
    let max_commits = contributors.iter().map(|c| c.commits).max().unwrap_or(1);
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
                        view! {
                            <div class="flex items-center gap-4">
                                <Avatar 
                                    src=contrib.user.avatar_url.clone() 
                                    alt=contrib.user.display_name.clone() 
                                    class="w-12 h-12"
                                />
                                <div class="flex-1">
                                    <div class="flex items-center justify-between mb-1">
                                        <span class="font-semibold">{contrib.user.display_name.clone()}</span>
                                        <span class="text-sm text-gray-600">
                                            {format!("{:.1}%", contrib.percentage)}
                                        </span>
                                    </div>
                                    <Progress 
                                        value=contrib.percentage as u32
                                        max=Some(100)
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

