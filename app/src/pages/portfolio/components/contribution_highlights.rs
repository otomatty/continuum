use crate::components::card::{Card, CardBody, CardTitle};
use crate::concepts::contribution::RepositoryContribution;
use leptos::prelude::*;

/**
 * ContributionHighlights Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/portfolio/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   ├─ app/src/components/card.rs
 *   └─ app/src/concepts/contribution/mod.rs
 */
#[component]
pub fn ContributionHighlights(contributions: Vec<RepositoryContribution>) -> impl IntoView {
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
