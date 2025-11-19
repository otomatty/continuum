use leptos::prelude::*;
use crate::components::{card::{Card, CardBody}, badge::Badge, badge::BadgeVariant, avatar::Avatar, table::{Table, TableHead, TableBody, TableRow, TableHeader, TableCell}};
use crate::mock::data::{RankingEntry, Activity, Repository, ActivityType};

#[component]
pub fn StatsCard(
    title: String,
    value: String,
    #[prop(optional)] description: &'static str,
) -> impl IntoView {
    view! {
        <Card>
            <CardBody>
                <h3 class="text-sm font-medium text-gray-500 mb-1">{title}</h3>
                <p class="text-3xl font-bold">{value}</p>
                {if !description.is_empty() {
                    Some(view! { <p class="text-sm text-gray-400 mt-2">{description}</p> })
                } else {
                    None
                }}
            </CardBody>
        </Card>
    }
}

#[component]
pub fn RankingTable(
    rankings: Vec<RankingEntry>,
) -> impl IntoView {
    view! {
        <Table class="w-full">
            <TableHead>
                <TableRow>
                    <TableHeader>"Rank"</TableHeader>
                    <TableHeader>"User"</TableHeader>
                    <TableHeader>"Commits"</TableHeader>
                    <TableHeader>"PRs"</TableHeader>
                    <TableHeader>"Reviews"</TableHeader>
                    <TableHeader>"Score"</TableHeader>
                </TableRow>
            </TableHead>
            <TableBody>
                {rankings.into_iter().map(|entry| {
                    view! {
                        <TableRow>
                            <TableCell>
                                <Badge variant=BadgeVariant::Primary>
                                    {format!("#{}", entry.rank)}
                                </Badge>
                            </TableCell>
                            <TableCell>
                                <div class="flex items-center gap-3">
                                    <Avatar src=entry.user.avatar_url.clone() alt=entry.user.display_name.clone() class="w-8 h-8" />
                                    <div>
                                        <div class="font-semibold">{entry.user.display_name.clone()}</div>
                                        <div class="text-sm text-gray-500">{entry.user.username.clone()}</div>
                                    </div>
                                </div>
                            </TableCell>
                            <TableCell>{entry.commits}</TableCell>
                            <TableCell>{entry.prs}</TableCell>
                            <TableCell>{entry.reviews}</TableCell>
                            <TableCell>
                                <span class="font-bold">{entry.score}</span>
                            </TableCell>
                        </TableRow>
                    }
                }).collect::<Vec<_>>()}
            </TableBody>
        </Table>
    }
}

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

