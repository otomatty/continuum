mod components;

use crate::components::{container::Container, fade_in::FadeIn, heading::SectionTitle};
use components::{
    AboutContinuum, CTASection, FeatureShowcase, FinalCTA, HomeFooter, SocialProof,
    StatisticsPreview, ValuePropositionCard,
};
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
 *   ├─ app/src/components/container/mod.rs
 *   ├─ app/src/components/heading/mod.rs
 *   └─ app/src/pages/home/components/mod.rs
 */
#[component]
pub fn HomePage() -> impl IntoView {
    #[cfg(feature = "hydrate")]
    {
        // Check authentication status and redirect if authenticated using use_auth hook
        use crate::hooks::use_auth;
        use web_sys::window;

        let auth = use_auth();

        Effect::new(move |_| {
            if let Some(status) = auth.status.get() {
                if status.authenticated {
                    if let Some(window) = window() {
                        let _ = window.location().set_href("/dashboard");
                    }
                }
            }
        });
    }

    view! {
        <main>
            // Hero Section with CTA
            <CTASection
                headline="エンジニアの成長を、組織全体で支援する".to_string()
                subheadline="OSS活動を通じた実践的な学習と知見共有のプラットフォーム".to_string()
            />

            // About Continuum Section
            <AboutContinuum />

            // Value Proposition Section (3 columns)
            <section class="py-32">
                <Container>
                    <FadeIn>
                        <SectionTitle title="Continuumが提供する価値".to_string() margin_bottom="mb-12" />
                    </FadeIn>
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                        <FadeIn delay="100".to_string()>
                            <ValuePropositionCard
                                title="OSS活動への参加を支援".to_string()
                                description="会社が支援することで、OSS活動のハードルを下げ、より多くの社員が参加しやすくなります。組織全体でエンジニアの成長を後押しします。".to_string()
                                icon="🚀".to_string()
                            />
                        </FadeIn>
                        <FadeIn delay="200".to_string()>
                            <ValuePropositionCard
                                title="実践的な学習の場".to_string()
                                description="社内エンジニア同士が知見を共有し、実践的な経験を通じてスキルを向上させることができます。最新の技術やベストプラクティスを学べる環境を提供します。".to_string()
                                icon="📚".to_string()
                            />
                        </FadeIn>
                        <FadeIn delay="300".to_string()>
                            <ValuePropositionCard
                                title="成長しやすい環境の提供".to_string()
                                description="エンジニアが働きやすく成長しやすい環境を提供するための一環として、ここでの活動が実績として記録・可視化されます。長く働き続けられる環境づくりを支援します。".to_string()
                                icon="🌱".to_string()
                            />
                        </FadeIn>
                    </div>
                </Container>
            </section>

            // Statistics Preview Section
            <section class="py-32">
                <Container>
                    <FadeIn>
                        <SectionTitle title="組織の活動状況".to_string() margin_bottom="mb-8" />
                    </FadeIn>
                    <StatisticsPreview
                        total_contributors=127
                        total_repositories=45
                        external_prs_this_month=23
                    />
                </Container>
            </section>

            // Feature Showcase Section
            <FeatureShowcase />

            // Social Proof Section
            <SocialProof />

            // Final CTA Section
            <FinalCTA />

            // Footer
            <HomeFooter />
        </main>
    }
}
