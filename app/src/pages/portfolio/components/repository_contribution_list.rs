use crate::components::card::{Card, CardBody, CardTitle};
use crate::concepts::contribution::RepositoryContribution;
use leptos::prelude::*;

/**
 * RepositoryContributionList Component
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
pub fn RepositoryContributionList(contributions: Vec<RepositoryContribution>) -> impl IntoView {
    view! {
        <Card>
            <CardTitle>"Repository Contributions"</CardTitle>
            <CardBody>
                <div class="space-y-4">
                    {contributions.into_iter().map(|contrib| {
                        view! {
                            <div class="border border-gray-200 rounded-lg p-4 hover:bg-gray-50">
                                <div class="flex justify-between items-start mb-3">
                                    <div>
                                        <h3 class="text-lg font-semibold">
                                            <a href=format!("https://github.com/{}", contrib.repository.full_name) target="_blank" class="hover:underline">
                                                {contrib.repository.name.clone()}
                                            </a>
                                        </h3>
                                        {contrib.repository.description.clone().map(|desc| view! { <p class="text-sm text-gray-600 mt-1">{desc}</p> })}
                                    </div>
                                    <div class="text-right">
                                        <div class="text-2xl font-bold text-blue-600">{format!("{:.1}%", contrib.percentage)}</div>
                                        <div class="text-sm text-gray-500">"Contribution"</div>
                                    </div>
                                </div>
                                <div class="grid grid-cols-2 md:grid-cols-5 gap-4 text-sm">
                                    <div>
                                        <div class="text-gray-500">"Commits"</div>
                                        <div class="font-semibold">{contrib.commits}</div>
                                    </div>
                                    <div>
                                        <div class="text-gray-500">"PRs"</div>
                                        <div class="font-semibold">{contrib.prs}</div>
                                    </div>
                                    <div>
                                        <div class="text-gray-500">"Reviews"</div>
                                        <div class="font-semibold">{contrib.reviews}</div>
                                    </div>
                                    <div>
                                        <div class="text-gray-500">"Lines Added"</div>
                                        <div class="font-semibold text-green-600">{contrib.lines_added}</div>
                                    </div>
                                    <div>
                                        <div class="text-gray-500">"Lines Deleted"</div>
                                        <div class="font-semibold text-red-600">{contrib.lines_deleted}</div>
                                    </div>
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </CardBody>
        </Card>
    }
}
