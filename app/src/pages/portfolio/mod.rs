mod components;

use leptos::prelude::*;
use leptos_router::params::Params;
use crate::mock::data::{
    generate_mock_users, generate_mock_contribution_graph,
    generate_mock_repository_contributions, Period
};
use components::{UserProfile, ContributionGraph, RepositoryContributionList, ContributionHighlights};

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
 *   └─ app/src/mock/data.rs
 *
 * Related Documentation:
 *   └─ docs/03_plans/continuum/prototype-pages.md
 */
#[component]
pub fn PortfolioPage() -> impl IntoView {
    // For now, use default username since path params routing needs to be fixed
    let username = move || "alice-dev".to_string();
    
    let users = generate_mock_users();
    let user = move || {
        users.iter()
            .find(|u| u.username == username())
            .cloned()
            .unwrap_or_else(|| users[0].clone())
    };
    
    let contribution_graph_weekly = move || {
        generate_mock_contribution_graph(&username(), Period::Weekly)
    };
    
    let contribution_graph_monthly = move || {
        generate_mock_contribution_graph(&username(), Period::Monthly)
    };
    
    let repository_contributions = move || {
        generate_mock_repository_contributions(&username())
    };
    
    let (selected_period, set_selected_period) = signal(Period::Weekly);
    
    view! {
        <div class="space-y-8">
            <UserProfile user=user() />
            
            <ContributionHighlights contributions=repository_contributions() />
            
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
                                    class=move || {
                                        let p = selected_period.get();
                                        format!("join-item btn btn-sm {}", if p == Period::Weekly { "btn-active" } else { "" })
                                    }
                                    on:click=move |_| set_selected_period.set(Period::Weekly)
                                >
                                    "Weekly"
                                </button>
                                <button
                                    class=move || {
                                        let p = selected_period.get();
                                        format!("join-item btn btn-sm {}", if p == Period::Monthly { "btn-active" } else { "" })
                                    }
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
            
            <RepositoryContributionList contributions=repository_contributions() />
        </div>
    }
}

