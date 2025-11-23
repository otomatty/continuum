use crate::components::{container::Container, heading::SectionTitle};
use leptos::prelude::*;

/**
 * SocialProof Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/home/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   ├─ leptos::prelude
 *   ├─ app/src/components/container.rs
 *   └─ app/src/components/heading/mod.rs
 */
#[component]
pub fn SocialProof() -> impl IntoView {
    view! {
        <section class="bg-base-200 py-16">
            <Container>
                <SectionTitle title="組織の活動実績".to_string() margin_bottom="mb-12" />

                // Top Contributors
                <div class="mb-12">
                    <h3 class="text-xl font-semibold mb-6 text-center">"活躍中のコントリビューター"</h3>
                    <div class="grid grid-cols-1 md:grid-cols-3 lg:grid-cols-5 gap-4">
                        {[1, 2, 3, 4, 5].into_iter().map(|i| {
                            view! {
                                <div class="card bg-base-100 shadow-sm hover:shadow-md transition-shadow">
                                    <div class="card-body items-center text-center p-4">
                                        <div class="avatar placeholder mb-2">
                                            <div class="bg-neutral text-neutral-content rounded-full w-16">
                                                <span class="text-xl">{format!("U{}", i)}</span>
                                            </div>
                                        </div>
                                        <h4 class="font-semibold">{"ユーザー"}{i}</h4>
                                        <p class="text-sm text-gray-500">"コミット: 150+"</p>
                                    </div>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>

                // Featured Repositories
                <div>
                    <h3 class="text-xl font-semibold mb-6 text-center">"注目のリポジトリ"</h3>
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                        {[
                            ("リポジトリA", "組織の主要プロジェクト", 234),
                            ("リポジトリB", "オープンソースライブラリ", 189),
                            ("リポジトリC", "開発ツール", 156),
                        ].into_iter().map(|(name, desc, stars)| {
                            view! {
                                <div class="card bg-base-100 shadow-sm hover:shadow-md transition-shadow">
                                    <div class="card-body">
                                        <h4 class="font-semibold text-lg mb-2">{name}</h4>
                                        <p class="text-sm text-gray-600 mb-4">{desc}</p>
                                        <div class="flex items-center gap-2">
                                            <span class="text-yellow-500">"⭐"</span>
                                            <span class="text-sm">{stars.to_string()}</span>
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>
            </Container>
        </section>
    }
}
