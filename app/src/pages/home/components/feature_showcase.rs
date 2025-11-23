use crate::components::{fade_in::FadeIn, heading::SectionTitle, section::Section};
use leptos::prelude::*;

/**
 * FeatureShowcase Component
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
pub fn FeatureShowcase() -> impl IntoView {
    view! {
        <Section>
            <FadeIn>
                <SectionTitle title="主要機能".to_string() margin_bottom="mb-16" />
            </FadeIn>

            // Feature 1: Dashboard
            <FadeIn delay="100".to_string()>
                <div class="flex flex-col md:flex-row items-center gap-12 mb-20">
                    <div class="flex-1">
                        <div class="text-4xl mb-4">"📊"</div>
                        <h3 class="text-2xl font-bold mb-4">"組織全体の活動を一目で"</h3>
                        <p class="text-gray-600 mb-6">
                            "リアルタイムのアクティビティタイムライン、週間/月間ランキング、リポジトリ一覧で、組織のOSS活動を可視化します。"
                        </p>
                        <p class="text-sm text-gray-500">
                            "ログイン後、ダッシュボードで組織全体の活動状況を確認できます。"
                        </p>
                    </div>
                    <div class="flex-1 bg-gray-100 rounded-lg p-8 min-h-[300px] flex items-center justify-center">
                        <p class="text-gray-400">"ダッシュボードのスクリーンショット"</p>
                    </div>
                </div>
            </FadeIn>

            // Feature 2: Portfolio
            <FadeIn delay="200".to_string()>
                <div class="flex flex-col md:flex-row-reverse items-center gap-12 mb-20">
                    <div class="flex-1">
                        <div class="text-4xl mb-4">"💼"</div>
                        <h3 class="text-2xl font-bold mb-4">"実績を永続的に記録"</h3>
                        <p class="text-gray-600 mb-6">
                            "個人ページで全コントリビューションを時系列やリポジトリ別に可視化。社外にも共有可能な公開URLで、実績として活用できます。"
                        </p>
                        <p class="text-sm text-gray-500">
                            "ここでの活動が実績として記録され、長期的に活用できます。"
                        </p>
                    </div>
                    <div class="flex-1 bg-gray-100 rounded-lg p-8 min-h-[300px] flex items-center justify-center">
                        <p class="text-gray-400">"ポートフォリオのスクリーンショット"</p>
                    </div>
                </div>
            </FadeIn>

            // Feature 3: Knowledge
            <FadeIn delay="300".to_string()>
                <div class="flex flex-col md:flex-row items-center gap-12">
                    <div class="flex-1">
                        <div class="text-4xl mb-4">"💡"</div>
                        <h3 class="text-2xl font-bold mb-4">"組織の知見を集約・共有"</h3>
                        <p class="text-gray-600 mb-6">
                            "GitHub Discussionsをデータソースとしたナレッジベース。カテゴリや投稿者による検索・フィルタリングで、必要な情報を素早く見つけられます。"
                        </p>
                        <p class="text-sm text-gray-500">
                            "社内エンジニア同士が知見を共有し、実践的な学習の場を提供します。"
                        </p>
                    </div>
                    <div class="flex-1 bg-gray-100 rounded-lg p-8 min-h-[300px] flex items-center justify-center">
                        <p class="text-gray-400">"ナレッジページのスクリーンショット"</p>
                    </div>
                </div>
            </FadeIn>
        </Section>
    }
}
