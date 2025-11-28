mod components;

use crate::components::auth_guard::AuthGuard;
use crate::concepts::contribution::{
    initialize_mock_contribution_graph, initialize_mock_repository_contributions,
    ContributionPeriod,
};
use crate::concepts::user::{get_user_by_username, initialize_mock_users};
use components::{
    ContributionGraphComponent, ContributionHighlights, RepositoryContributionDisplay,
    RepositoryContributionList, UserProfile,
};
use leptos::prelude::*;
use leptos_router::params::Params;

#[derive(Params, PartialEq, Clone)]
pub struct PortfolioParams {
    username: Option<String>,
}

/**
 * Portfolio Page
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   └─ app/src/lib.rs (ルーティング)
 *
 * Dependencies (External files that this file imports):
 *   ├─ app/src/components/auth_guard/mod.rs
 *   ├─ app/src/components/card.rs
 *   ├─ app/src/components/badge.rs
 *   ├─ app/src/components/avatar.rs
 *   ├─ app/src/concepts/user/mod.rs
 *   └─ app/src/concepts/contribution/mod.rs
 *
 * Related Documentation:
 *   └─ docs/03_plans/continuum/prototype-pages.md
 */
#[component]
pub fn PortfolioPage() -> impl IntoView {
    view! {
        <AuthGuard>
            <PortfolioContent />
        </AuthGuard>
    }
}

/// Portfolio content component (requires authentication)
#[component]
fn PortfolioContent() -> impl IntoView {
    // For now, use default username since path params routing needs to be fixed
    // TODO: Replace with proper path parameter routing when available. See issue #123.
    let username = move || "alice-dev".to_string();

    let user_state = initialize_mock_users();
    let user = move || {
        get_user_by_username(&user_state, &username())
            .unwrap_or_else(|| user_state.users[0].clone())
    };

    let contribution_graph_weekly =
        move || initialize_mock_contribution_graph(&username(), ContributionPeriod::Weekly);

    let contribution_graph_monthly =
        move || initialize_mock_contribution_graph(&username(), ContributionPeriod::Monthly);

    // Generate repository contributions once before view! macro to avoid regeneration
    let repository_contributions = initialize_mock_repository_contributions(&username());

    // Convert to display format for RepositoryContributionList
    let repository_contributions_display: Vec<RepositoryContributionDisplay> =
        repository_contributions
            .iter()
            .map(|c| RepositoryContributionDisplay::from_contribution_with_mock(c.clone()))
            .collect();

    let (selected_period, set_selected_period) = signal(ContributionPeriod::Weekly);

    view! {
        <div class="space-y-8">
            <UserProfile user=user() />

            <ContributionHighlights contributions=repository_contributions.clone() />

            {move || {
                let period = selected_period.get();
                let graph = if period == ContributionPeriod::Weekly {
                    contribution_graph_weekly()
                } else {
                    contribution_graph_monthly()
                };
                view! {
                    <div>
                        <div class="flex justify-between items-center mb-4">
                            <h2 class="text-2xl font-bold">"Contribution Graph"</h2>
                            <div class="join">
                                <button
                                    class="join-item btn btn-sm"
                                    class:btn-active=move || selected_period.get() == ContributionPeriod::Weekly
                                    on:click=move |_| set_selected_period.set(ContributionPeriod::Weekly)
                                >
                                    "Weekly"
                                </button>
                                <button
                                    class="join-item btn btn-sm"
                                    class:btn-active=move || selected_period.get() == ContributionPeriod::Monthly
                                    on:click=move |_| set_selected_period.set(ContributionPeriod::Monthly)
                                >
                                    "Monthly"
                                </button>
                            </div>
                        </div>
                        <ContributionGraphComponent graph=graph />
                    </div>
                }
            }}

            <RepositoryContributionList contributions=repository_contributions_display.clone() />
        </div>
    }
}
