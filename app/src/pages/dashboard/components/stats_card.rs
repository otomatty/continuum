use leptos::prelude::*;
use crate::components::card::{Card, CardBody};

/**
 * StatsCard Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/dashboard/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   └─ app/src/components/card.rs
 */
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

