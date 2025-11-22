use crate::components::{
    avatar::Avatar,
    badge::{Badge, BadgeVariant},
    table::{Table, TableBody, TableCell, TableHead, TableHeader, TableRow},
};
use crate::concepts::ranking::RankingEntry;
use leptos::prelude::*;

/**
 * RankingTable Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/dashboard/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   ├─ app/src/components/badge.rs
 *   ├─ app/src/components/avatar.rs
 *   ├─ app/src/components/table.rs
 *   └─ app/src/concepts/ranking/mod.rs
 */
#[component]
pub fn RankingTable(rankings: Vec<RankingEntry>) -> impl IntoView {
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
