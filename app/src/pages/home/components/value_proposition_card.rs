use crate::components::card::{Card, CardBody, CardTitle};
use leptos::prelude::*;

/**
 * ValuePropositionCard Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/home/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   └─ app/src/components/card.rs
 */
#[component]
pub fn ValuePropositionCard(
    title: String,
    description: String,
    #[prop(optional)] icon: Option<String>,
) -> impl IntoView {
    view! {
        <Card class="h-full">
            <CardBody>
                {icon.map(|icon_class| {
                    view! {
                        <div class="text-4xl mb-4">{icon_class}</div>
                    }
                })}
                <CardTitle class="text-xl mb-3">{title}</CardTitle>
                <p class="text-gray-600">{description}</p>
            </CardBody>
        </Card>
    }
}
