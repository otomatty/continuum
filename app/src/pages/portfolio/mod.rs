mod components;

use crate::concepts::contribution::{
    initialize_mock_contribution_graph, initialize_mock_repository_contributions,
};
use crate::concepts::organization::Period;
use crate::concepts::user::{get_user_by_username, initialize_mock_users};
use components::{
    ContributionGraph, ContributionHighlights, RepositoryContributionList, UserProfile,
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
 *   ├─ app/src/components/card.rs
 *   ├─ app/src/components/badge.rs
 *   ├─ app/src/components/avatar.rs
 *   ├─ app/src/concepts/user/mod.rs
 *   ├─ app/src/concepts/contribution/mod.rs
 *   └─ app/src/concepts/organization/mod.rs
 *
 * Related Documentation:
 *   └─ docs/03_plans/continuum/prototype-pages.md
 */
#[component]
pub fn PortfolioPage() -> impl IntoView {
    // For now, use default username since path params routing needs to be fixed
    // TODO: Replace with proper path parameter routing when available. See issue #123.
    let username = move || "alice-dev".to_string();

    let user_state = initialize_mock_users();
    let user = move || {
        get_user_by_username(&user_state, &username())
            .unwrap_or_else(|| user_state.users[0].clone())
    };

    let contribution_graph_weekly =
        move || initialize_mock_contribution_graph(&username(), Period::Weekly);

    let contribution_graph_monthly =
        move || initialize_mock_contribution_graph(&username(), Period::Monthly);

    // Generate repository contributions once before view! macro to avoid regeneration
    let repository_contributions = initialize_mock_repository_contributions(&username());

    let (selected_period, set_selected_period) = signal(Period::Weekly);

    view! {
        <div class="space-y-8">
            <UserProfile user=user() />

            <ContributionHighlights contributions=repository_contributions.clone() />

            {move || {
                let period = selected_period.get();
                let graph = if period == Period::Weekly {
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
                        <ContributionGraph graph=graph />
                    </div>
                }
            }}

            <RepositoryContributionList contributions=repository_contributions.clone() />
        </div>
    }
}
