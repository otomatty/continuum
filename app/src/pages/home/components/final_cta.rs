use crate::components::{
    fade_in::FadeIn, github_login_button::GitHubLoginButton, section::Section,
};
use leptos::prelude::*;

/**
 * FinalCTA Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/home/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   ├─ app/src/components/github_login_button/mod.rs
 *   ├─ app/src/components/section/mod.rs
 *   └─ app/src/components/fade_in/mod.rs
 */
#[component]
pub fn FinalCTA() -> impl IntoView {
    view! {
        <Section
            background="bg-gradient-to-br from-primary/10 to-secondary/10".to_string()
        >
            <FadeIn>
                <div class="max-w-2xl mx-auto text-center">
                    <h2 class="text-3xl font-bold mb-4">"今すぐ始めて、成長を加速させる"</h2>
                    <p class="text-lg text-gray-600 mb-8">
                        "社内エンジニア向けの成長支援ツール。GitHubアカウントがあれば、すぐに始められます。"
                    </p>
                    <GitHubLoginButton />
                    <p class="text-sm text-gray-500 mt-4">
                        "組織のエンジニアが働きやすく成長しやすい環境を提供するための一環です"
                    </p>
                </div>
            </FadeIn>
        </Section>
    }
}
