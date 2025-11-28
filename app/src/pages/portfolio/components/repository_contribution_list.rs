// RepositoryContributionList Component
//
// DEPENDENCY MAP:
//
// Parents (Files that import this component):
//   └─ app/src/pages/portfolio/mod.rs
//
// Dependencies (External files that this component imports):
//   ├─ app/src/components/card.rs
//   └─ app/src/concepts/contribution/mod.rs
//
// Note: RepositoryContribution は ID 参照のみ保持するため、
// 表示に必要な情報は RepositoryContributionDisplay として UI 層で結合する

use crate::components::card::{Card, CardBody, CardTitle};
use crate::concepts::contribution::RepositoryContribution;
use leptos::prelude::*;

/// UI 表示用の RepositoryContribution データ（Synchronization/UI 層で結合）
#[derive(Debug, Clone)]
pub struct RepositoryContributionDisplay {
    pub contribution: RepositoryContribution,
    pub repository_name: String,
    pub repository_full_name: String,
    pub repository_description: Option<String>,
}

impl RepositoryContributionDisplay {
    /// モックデータから表示用データを生成（開発用）
    pub fn from_contribution_with_mock(contribution: RepositoryContribution) -> Self {
        let repository_name = contribution.repository_id.clone();
        let repository_full_name = format!("org/{}", contribution.repository_id);
        let repository_description =
            Some(format!("Description for {}", contribution.repository_id));

        Self {
            contribution,
            repository_name,
            repository_full_name,
            repository_description,
        }
    }
}

#[component]
pub fn RepositoryContributionList(
    contributions: Vec<RepositoryContributionDisplay>,
) -> impl IntoView {
    view! {
        <Card>
            <CardTitle>"Repository Contributions"</CardTitle>
            <CardBody>
                <div class="space-y-4">
                    {contributions.into_iter().map(|contrib_display| {
                        view! {
                            <div class="border border-gray-200 rounded-lg p-4 hover:bg-gray-50">
                                <div class="flex justify-between items-start mb-3">
                                    <div>
                                        <h3 class="text-lg font-semibold">
                                            <a href=format!("https://github.com/{}", contrib_display.repository_full_name) target="_blank" class="hover:underline">
                                                {contrib_display.repository_name.clone()}
                                            </a>
                                        </h3>
                                        {contrib_display.repository_description.clone().map(|desc| view! { <p class="text-sm text-gray-600 mt-1">{desc}</p> })}
                                    </div>
                                    <div class="text-right">
                                        <div class="text-2xl font-bold text-blue-600">{format!("{:.1}%", contrib_display.contribution.percentage)}</div>
                                        <div class="text-sm text-gray-500">"Contribution"</div>
                                    </div>
                                </div>
                                <div class="grid grid-cols-2 md:grid-cols-5 gap-4 text-sm">
                                    <div>
                                        <div class="text-gray-500">"Commits"</div>
                                        <div class="font-semibold">{contrib_display.contribution.commits}</div>
                                    </div>
                                    <div>
                                        <div class="text-gray-500">"PRs"</div>
                                        <div class="font-semibold">{contrib_display.contribution.prs}</div>
                                    </div>
                                    <div>
                                        <div class="text-gray-500">"Reviews"</div>
                                        <div class="font-semibold">{contrib_display.contribution.reviews}</div>
                                    </div>
                                    <div>
                                        <div class="text-gray-500">"Lines Added"</div>
                                        <div class="font-semibold text-green-600">{contrib_display.contribution.lines_added}</div>
                                    </div>
                                    <div>
                                        <div class="text-gray-500">"Lines Deleted"</div>
                                        <div class="font-semibold text-red-600">{contrib_display.contribution.lines_deleted}</div>
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
