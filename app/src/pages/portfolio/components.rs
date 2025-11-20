use leptos::prelude::*;
use crate::components::{card::{Card, CardTitle, CardBody}, badge::{Badge, BadgeVariant}, avatar::Avatar};
use crate::mock::data::{User, ContributionGraph, RepositoryContribution, UserRole, Period};

#[component]
pub fn UserProfile(
    user: User,
) -> impl IntoView {
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

#[component]
pub fn ContributionGraph(
    graph: ContributionGraph,
) -> impl IntoView {
    let max_commits = graph.data.iter().map(|d| d.commits).max().unwrap_or(1) as f64;
    let max_prs = graph.data.iter().map(|d| d.prs).max().unwrap_or(1) as f64;
    let max_reviews = graph.data.iter().map(|d| d.reviews).max().unwrap_or(1) as f64;
    
    view! {
        <Card>
            <CardTitle>
                {format!("Contribution Graph ({})", match graph.period {
                    Period::Weekly => "Weekly",
                    Period::Monthly => "Monthly",
                    Period::All => "All Time",
                })}
            </CardTitle>
            <CardBody>
                <div class="overflow-x-auto">
                    <div class="min-w-full">
                        <div class="grid grid-cols-7 md:grid-cols-14 lg:grid-cols-30 gap-1">
                            {graph.data.iter().map(|day| {
                                let intensity = ((day.commits + day.prs + day.reviews) as f64 / (max_commits + max_prs + max_reviews) * 4.0) as u32;
                                let bg_color = match intensity {
                                    0 => "bg-gray-100",
                                    1 => "bg-green-200",
                                    2 => "bg-green-400",
                                    3 => "bg-green-600",
                                    _ => "bg-green-800",
                                };
                                
                                view! {
                                    <div 
                                        class=format!("{} w-3 h-3 rounded-sm", bg_color)
                                        title=format!("{}: {} commits, {} PRs, {} reviews", 
                                            day.date.format("%Y-%m-%d").to_string(),
                                            day.commits,
                                            day.prs,
                                            day.reviews
                                        )
                                    ></div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                    <div class="flex items-center justify-end gap-4 mt-4 text-sm text-gray-500">
                        <span>"Less"</span>
                        <div class="flex gap-1">
                            <div class="w-3 h-3 rounded-sm bg-gray-100"></div>
                            <div class="w-3 h-3 rounded-sm bg-green-200"></div>
                            <div class="w-3 h-3 rounded-sm bg-green-400"></div>
                            <div class="w-3 h-3 rounded-sm bg-green-600"></div>
                            <div class="w-3 h-3 rounded-sm bg-green-800"></div>
                        </div>
                        <span>"More"</span>
                    </div>
                </div>
            </CardBody>
        </Card>
    }
}

#[component]
pub fn RepositoryContributionList(
    contributions: Vec<RepositoryContribution>,
) -> impl IntoView {
    view! {
        <Card>
            <CardTitle>"Repository Contributions"</CardTitle>
            <CardBody>
                <div class="space-y-4">
                    {contributions.into_iter().map(|contrib| {
                        view! {
                            <div class="border border-gray-200 rounded-lg p-4 hover:bg-gray-50">
                                <div class="flex justify-between items-start mb-3">
                                    <div>
                                        <h3 class="text-lg font-semibold">
                                            <a href=format!("https://github.com/{}", contrib.repository.full_name) target="_blank" class="hover:underline">
                                                {contrib.repository.name.clone()}
                                            </a>
                                        </h3>
                                        {contrib.repository.description.clone().map(|desc| view! { <p class="text-sm text-gray-600 mt-1">{desc}</p> })}
                                    </div>
                                    <div class="text-right">
                                        <div class="text-2xl font-bold text-blue-600">{format!("{:.1}%", contrib.percentage)}</div>
                                        <div class="text-sm text-gray-500">"Contribution"</div>
                                    </div>
                                </div>
                                <div class="grid grid-cols-2 md:grid-cols-5 gap-4 text-sm">
                                    <div>
                                        <div class="text-gray-500">"Commits"</div>
                                        <div class="font-semibold">{contrib.commits}</div>
                                    </div>
                                    <div>
                                        <div class="text-gray-500">"PRs"</div>
                                        <div class="font-semibold">{contrib.prs}</div>
                                    </div>
                                    <div>
                                        <div class="text-gray-500">"Reviews"</div>
                                        <div class="font-semibold">{contrib.reviews}</div>
                                    </div>
                                    <div>
                                        <div class="text-gray-500">"Lines Added"</div>
                                        <div class="font-semibold text-green-600">{contrib.lines_added}</div>
                                    </div>
                                    <div>
                                        <div class="text-gray-500">"Lines Deleted"</div>
                                        <div class="font-semibold text-red-600">{contrib.lines_deleted}</div>
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

#[component]
pub fn ContributionHighlights(
    contributions: Vec<RepositoryContribution>,
) -> impl IntoView {
    let (total_commits, total_prs, total_reviews, total_lines_added, total_lines_deleted) =
        contributions.iter().fold((0, 0, 0, 0, 0), |acc, c| {
            (
                acc.0 + c.commits,
                acc.1 + c.prs,
                acc.2 + c.reviews,
                acc.3 + c.lines_added,
                acc.4 + c.lines_deleted,
            )
        });
    
    view! {
        <Card>
            <CardTitle>"Organization Contribution Highlights"</CardTitle>
            <CardBody>
                <div class="grid grid-cols-2 md:grid-cols-5 gap-4">
                    <div class="text-center p-4 bg-blue-50 rounded-lg">
                        <div class="text-3xl font-bold text-blue-600">{total_commits}</div>
                        <div class="text-sm text-gray-600 mt-1">"Total Commits"</div>
                    </div>
                    <div class="text-center p-4 bg-purple-50 rounded-lg">
                        <div class="text-3xl font-bold text-purple-600">{total_prs}</div>
                        <div class="text-sm text-gray-600 mt-1">"Pull Requests"</div>
                    </div>
                    <div class="text-center p-4 bg-green-50 rounded-lg">
                        <div class="text-3xl font-bold text-green-600">{total_reviews}</div>
                        <div class="text-sm text-gray-600 mt-1">"Reviews"</div>
                    </div>
                    <div class="text-center p-4 bg-emerald-50 rounded-lg">
                        <div class="text-3xl font-bold text-emerald-600">{total_lines_added}</div>
                        <div class="text-sm text-gray-600 mt-1">"Lines Added"</div>
                    </div>
                    <div class="text-center p-4 bg-red-50 rounded-lg">
                        <div class="text-3xl font-bold text-red-600">{total_lines_deleted}</div>
                        <div class="text-sm text-gray-600 mt-1">"Lines Deleted"</div>
                    </div>
                </div>
            </CardBody>
        </Card>
    }
}

