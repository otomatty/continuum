use leptos::prelude::*;
use crate::components::card::{Card, CardTitle, CardBody};
use crate::concepts::repository::ContributorStats;

/**
 * ContributorPieChart Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/repository/components/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   ├─ app/src/components/card.rs
 *   └─ app/src/concepts/repository/mod.rs
 */
#[component]
pub fn ContributorPieChart(
    contributors: Vec<ContributorStats>,
) -> impl IntoView {
    let total_commits: u32 = contributors.iter().map(|c| c.commits).sum();
    
    // Calculate angles for pie chart
    let mut cumulative_percentage = 0.0;
    let pie_segments: Vec<_> = contributors.iter().map(|contrib| {
        let percentage = contrib.percentage;
        let start_angle = cumulative_percentage * 360.0;
        cumulative_percentage += percentage / 100.0;
        let end_angle = cumulative_percentage * 360.0;
        (contrib.clone(), start_angle, end_angle, percentage)
    }).collect();

    // Generate colors for segments
    let colors = vec![
        "#3b82f6", // blue
        "#10b981", // green
        "#f59e0b", // amber
        "#ef4444", // red
        "#8b5cf6", // purple
        "#ec4899", // pink
        "#06b6d4", // cyan
        "#84cc16", // lime
    ];

    view! {
        <Card>
            <CardTitle>"Contributor Distribution"</CardTitle>
            <CardBody>
                <div class="flex flex-col md:flex-row gap-6 items-center">
                    <div class="relative w-48 h-48">
                        <svg viewBox="0 0 100 100" class="w-full h-full transform -rotate-90">
                            {pie_segments.iter().enumerate().map(|(idx, (_contrib, start_angle, end_angle, percentage))| {
                                let color = colors[idx % colors.len()];
                                let large_arc = if percentage > &50.0 { 1 } else { 0 };
                                
                                // Convert angles to radians and calculate coordinates
                                let start_rad = start_angle.to_radians();
                                let end_rad = end_angle.to_radians();
                                
                                let x1 = 50.0 + 50.0 * start_rad.cos();
                                let y1 = 50.0 + 50.0 * start_rad.sin();
                                let x2 = 50.0 + 50.0 * end_rad.cos();
                                let y2 = 50.0 + 50.0 * end_rad.sin();
                                
                                let path_d = format!(
                                    "M 50 50 L {} {} A 50 50 0 {} 1 {} {} Z",
                                    x1, y1, large_arc, x2, y2
                                );
                                
                                view! {
                                    <path
                                        d=path_d
                                        fill=color
                                        stroke="white"
                                        stroke-width="1"
                                    />
                                }
                            }).collect::<Vec<_>>()}
                        </svg>
                    </div>
                    <div class="flex-1 space-y-2">
                        <h3 class="font-semibold mb-2">"Top Contributors"</h3>
                        {pie_segments.iter().enumerate().map(|(idx, (contrib, _, _, percentage))| {
                            let color = colors[idx % colors.len()];
                            view! {
                                <div class="flex items-center gap-2">
                                    <div 
                                        class="w-4 h-4 rounded"
                                        style=format!("background-color: {}", color)
                                    ></div>
                                    <span class="text-sm flex-1">{contrib.user.display_name.clone()}</span>
                                    <span class="text-sm font-semibold">{format!("{:.1}%", percentage)}</span>
                                    <span class="text-xs text-gray-500">({contrib.commits} commits)</span>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                        <div class="mt-4 pt-4 border-t">
                            <p class="text-sm text-gray-600">
                                "Total: " <span class="font-semibold">{total_commits}</span> " commits"
                            </p>
                        </div>
                    </div>
                </div>
            </CardBody>
        </Card>
    }
}

