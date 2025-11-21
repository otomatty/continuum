use leptos::prelude::*;
use crate::components::card::{Card, CardTitle, CardBody};
use crate::concepts::contribution::ContributionGraph;
use crate::concepts::organization::Period;

/**
 * ContributionGraph Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/portfolio/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   ├─ app/src/components/card.rs
 *   ├─ app/src/concepts/contribution/mod.rs
 *   └─ app/src/concepts/organization/mod.rs
 */
#[component]
pub fn ContributionGraph(
    graph: ContributionGraph,
) -> impl IntoView {
    let max_commits = graph.data.iter().map(|d| d.commits).max().unwrap_or(1) as f64;
    let max_prs = graph.data.iter().map(|d| d.prs).max().unwrap_or(1) as f64;
    let max_reviews = graph.data.iter().map(|d| d.reviews).max().unwrap_or(1) as f64;
    
    view! {
        <Card>
            <CardTitle>
                {format!("Contribution Graph ({})", match graph.period {
                    Period::Weekly => "Weekly",
                    Period::Monthly => "Monthly",
                    Period::All => "All Time",
                })}
            </CardTitle>
            <CardBody>
                <div class="overflow-x-auto">
                    <div class="min-w-full">
                        <div class="grid grid-cols-7 md:grid-cols-14 lg:grid-cols-30 gap-1">
                            {graph.data.iter().map(|day| {
                                let intensity = ((day.commits + day.prs + day.reviews) as f64 / (max_commits + max_prs + max_reviews) * 4.0) as u32;
                                let bg_color = match intensity {
                                    0 => "bg-gray-100",
                                    1 => "bg-green-200",
                                    2 => "bg-green-400",
                                    3 => "bg-green-600",
                                    _ => "bg-green-800",
                                };
                                
                                view! {
                                    <div 
                                        class=format!("{} w-3 h-3 rounded-sm", bg_color)
                                        title=format!("{}: {} commits, {} PRs, {} reviews", 
                                            day.date.format("%Y-%m-%d").to_string(),
                                            day.commits,
                                            day.prs,
                                            day.reviews
                                        )
                                    ></div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                    <div class="flex items-center justify-end gap-4 mt-4 text-sm text-gray-500">
                        <span>"Less"</span>
                        <div class="flex gap-1">
                            <div class="w-3 h-3 rounded-sm bg-gray-100"></div>
                            <div class="w-3 h-3 rounded-sm bg-green-200"></div>
                            <div class="w-3 h-3 rounded-sm bg-green-400"></div>
                            <div class="w-3 h-3 rounded-sm bg-green-600"></div>
                            <div class="w-3 h-3 rounded-sm bg-green-800"></div>
                        </div>
                        <span>"More"</span>
                    </div>
                </div>
            </CardBody>
        </Card>
    }
}

