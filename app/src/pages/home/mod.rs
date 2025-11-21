mod components;

use components::{CTASection, StatisticsPreview, ValuePropositionCard};
use leptos::prelude::*;

/**
 * HomePage Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/lib.rs
 *
 * Dependencies (External files that this component imports):
 *   └─ app/src/pages/home/components/mod.rs
 */
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="space-y-16">
            // Hero Section with CTA
            <CTASection
                headline="あなたのOSS活動を、永続的な資産に".to_string()
                subheadline="会社の枠を超えて輝く、あなたの技術実績を可視化".to_string()
                button_text="GitHub OAuth でログイン".to_string()
                button_href="/auth/github".to_string()
            />

            // Value Proposition Section (3 columns)
            <section class="container mx-auto px-4">
                <h2 class="text-3xl font-bold text-center mb-12">"Continuumの価値"</h2>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                    <ValuePropositionCard
                        title="永続的な実績の構築".to_string()
                        description="会社の枠を超えて、あなたのOSS活動を永続的な資産として記録・可視化します。転職や異動があっても、あなたの技術実績は失われません。".to_string()
                    />
                    <ValuePropositionCard
                        title="最先端技術でスキルアップ".to_string()
                        description="最新のOSSプロジェクトに参加することで、最先端の技術やベストプラクティスを学べます。実践的な経験を通じて、スキルを継続的に向上させることができます。".to_string()
                    />
                    <ValuePropositionCard
                        title="オープンな文化の醸成".to_string()
                        description="組織全体でOSS活動を促進し、オープンな文化を醸成します。社内外のコントリビューターと協力し、より良いソフトウェアを一緒に作り上げましょう。".to_string()
                    />
                </div>
            </section>

            // Statistics Preview Section
            <section class="container mx-auto px-4">
                <h2 class="text-3xl font-bold text-center mb-8">"組織の活動状況"</h2>
                <StatisticsPreview
                    total_contributors=127
                    total_repositories=45
                    external_prs_this_month=23
                />
            </section>
        </div>
    }
}
