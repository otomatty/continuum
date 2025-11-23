use crate::components::{fade_in::FadeIn, heading::SectionTitle, section::Section};
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
 *   ├─ app/src/components/section/mod.rs
 *   ├─ app/src/components/fade_in/mod.rs
 *   └─ app/src/components/heading/mod.rs
 */
#[component]
pub fn SocialProof() -> impl IntoView {
    view! {
        <Section background="bg-base-200".to_string()>
                <FadeIn>
                    <SectionTitle title="組織の活動実績".to_string() margin_bottom="mb-12" />
                </FadeIn>

                // Top Contributors
                <FadeIn delay="100".to_string()>
                    <div class="mb-12">
                        <h3 class="text-xl font-semibold mb-6 text-center">"活躍中のコントリビューター"</h3>
                        <div class="grid grid-cols-1 md:grid-cols-3 lg:grid-cols-5 gap-4">
                            {[1, 2, 3, 4, 5].into_iter().enumerate().map(|(idx, i)| {
                                let delay = (idx + 1) * 50;
                                view! {
                                    <FadeIn delay=format!("{}", delay)>
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
                                    </FadeIn>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                </FadeIn>

                // Featured Repositories
                <FadeIn delay="300".to_string()>
                    <div>
                        <h3 class="text-xl font-semibold mb-6 text-center">"注目のリポジトリ"</h3>
                        <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                            {[
                                ("リポジトリA", "組織の主要プロジェクト", 234),
                                ("リポジトリB", "オープンソースライブラリ", 189),
                                ("リポジトリC", "開発ツール", 156),
                            ].into_iter().enumerate().map(|(idx, (name, desc, stars))| {
                                let delay = 400 + (idx * 100);
                                view! {
                                    <FadeIn delay=format!("{}", delay)>
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
                                    </FadeIn>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                </FadeIn>
        </Section>
    }
}
