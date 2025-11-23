use crate::components::{
    fade_in::FadeIn,
    github_login_button::GitHubLoginButton,
    hero::{Hero, HeroContent},
};
use leptos::prelude::*;

/**
 * CTASection Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/home/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   ├─ app/src/components/hero.rs
 *   └─ app/src/components/github_login_button/mod.rs
 */
#[component]
pub fn CTASection(headline: String, subheadline: String, button_text: String) -> impl IntoView {
    view! {
        <Hero class="bg-gradient-to-br from-base-200 to-base-300 py-20 md:py-32">
            <HeroContent class="text-center">
                <FadeIn class="max-w-3xl mx-auto".to_string()>
                    <h1 class="text-4xl md:text-5xl lg:text-6xl font-bold mb-6 text-base-content">
                        {headline}
                    </h1>
                    <p class="text-lg md:text-xl text-base-content opacity-70 mb-10 leading-relaxed">
                        {subheadline}
                    </p>
                    <div class="inline-block transform transition-transform hover:scale-105">
                        <GitHubLoginButton />
                    </div>
                </FadeIn>
            </HeroContent>
        </Hero>
    }
}
