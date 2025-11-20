mod components;

use leptos::prelude::*;
use crate::components::card::{Card, CardTitle, CardBody};
use crate::concepts::organization::{initialize_mock_organization_stats, Period};
use crate::concepts::user::initialize_mock_users;
use crate::concepts::activity::initialize_mock_activities;
use crate::concepts::repository::initialize_mock_repositories;
use crate::synchronizations::{calculate_weekly_ranking, calculate_monthly_ranking};
use components::{StatsCard, RankingTable, ActivityTimeline, RepositoryList};

/**
 * Dashboard Page
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   └─ app/src/lib.rs (ルーティング)
 *
 * Dependencies (External files that this file imports):
 *   ├─ app/src/components/card.rs
 *   ├─ app/src/components/table.rs
 *   ├─ app/src/components/badge.rs
 *   ├─ app/src/components/avatar.rs
 *   ├─ app/src/concepts/organization/mod.rs
 *   ├─ app/src/concepts/user/mod.rs
 *   ├─ app/src/concepts/activity/mod.rs
 *   ├─ app/src/concepts/repository/mod.rs
 *   └─ app/src/synchronizations/ranking_sync.rs
 *
 * Related Documentation:
 *   └─ docs/03_plans/continuum/prototype-pages.md
 */
#[component]
pub fn DashboardPage() -> impl IntoView {
    let weekly_stats = initialize_mock_organization_stats(Period::Weekly);
    
    // Initialize Concept states
    let user_state = initialize_mock_users();
    let activity_state = initialize_mock_activities();
    
    // Use Synchronization to calculate rankings
    let weekly_ranking = calculate_weekly_ranking(&user_state, &activity_state);
    let monthly_ranking = calculate_monthly_ranking(&user_state, &activity_state);
    
    let activities = activity_state.activities;
    let repositories = initialize_mock_repositories().repositories;
    
    let (selected_period, set_selected_period) = signal(Period::Weekly);
    
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-4xl font-bold mb-2">"Dashboard"</h1>
                <p class="text-gray-600">"Organization activity overview"</p>
            </div>

            // Stats Cards
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
                <StatsCard 
                    title="Total Contributors".to_string()
                    value=weekly_stats.total_contributors.to_string()
                    description="Active this week"
                />
                <StatsCard 
                    title="Total Repositories".to_string()
                    value=weekly_stats.total_repositories.to_string()
                    description="Public repositories"
                />
                <StatsCard 
                    title="External PRs".to_string()
                    value=weekly_stats.external_prs_count.to_string()
                    description="This week"
                />
                <StatsCard 
                    title="Total Commits".to_string()
                    value=weekly_stats.total_commits.to_string()
                    description="This week"
                />
            </div>

            // Ranking Section
            <Card>
                <CardTitle>
                    <div class="flex justify-between items-center">
                        <span>"Activity Ranking"</span>
                        <div class="join">
                            <button
                                class="join-item btn btn-sm"
                                class:btn-active=move || selected_period.get() == Period::Weekly
                                on:click=move |_| set_selected_period.set(Period::Weekly)
                            >
                                "Weekly"
                            </button>
                            <button
                                class="join-item btn btn-sm"
                                class:btn-active=move || selected_period.get() == Period::Monthly
                                on:click=move |_| set_selected_period.set(Period::Monthly)
                            >
                                "Monthly"
                            </button>
                        </div>
                    </div>
                </CardTitle>
                <CardBody>
                    {move || {
                        let period = selected_period.get();
                        let ranking = if period == Period::Weekly {
                            weekly_ranking.clone()
                        } else {
                            monthly_ranking.clone()
                        };
                        view! {
                            <RankingTable rankings=ranking />
                        }
                    }}
                </CardBody>
            </Card>

            // Activity Timeline
            <Card>
                <CardTitle>"Recent Activity"</CardTitle>
                <CardBody>
                    <ActivityTimeline activities=activities />
                </CardBody>
            </Card>

            // Repository List
            <Card>
                <CardTitle>"Repositories"</CardTitle>
                <CardBody>
                    <RepositoryList repositories=repositories />
                </CardBody>
            </Card>
        </div>
    }
}

