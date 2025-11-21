use leptos::prelude::*;
use crate::components::stats::{Stats, StatItem, StatTitle, StatValue, StatDescription};

/**
 * StatisticsPreview Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/home/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   └─ app/src/components/stats.rs
 */
#[component]
pub fn StatisticsPreview(
    total_contributors: u32,
    total_repositories: u32,
    external_prs_this_month: u32,
) -> impl IntoView {
    view! {
        <Stats class="w-full">
            <StatItem>
                <StatTitle>"Total Contributors"</StatTitle>
                <StatValue>{total_contributors.to_string()}</StatValue>
                <StatDescription>"Active contributors in the organization"</StatDescription>
            </StatItem>
            <StatItem>
                <StatTitle>"Active Repositories"</StatTitle>
                <StatValue>{total_repositories.to_string()}</StatValue>
                <StatDescription>"Repositories with recent activity"</StatDescription>
            </StatItem>
            <StatItem>
                <StatTitle>"External PRs"</StatTitle>
                <StatValue>{external_prs_this_month.to_string()}</StatValue>
                <StatDescription>"This month"</StatDescription>
            </StatItem>
        </Stats>
    }
}

