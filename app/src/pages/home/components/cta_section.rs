use crate::components::{
    button::{Button, ButtonVariant},
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
 *   └─ app/src/components/button.rs
 */
#[component]
pub fn CTASection(
    headline: String,
    subheadline: String,
    button_text: String,
    button_href: String,
) -> impl IntoView {
    view! {
        <Hero class="bg-gradient-to-br from-base-200 to-base-300 py-20 md:py-32">
            <HeroContent class="text-center">
                <div class="max-w-3xl mx-auto animate-fade-in">
                    <h1 class="text-4xl md:text-5xl lg:text-6xl font-bold mb-6 text-base-content">
                        {headline}
                    </h1>
                    <p class="text-lg md:text-xl text-base-content opacity-70 mb-10 leading-relaxed">
                        {subheadline}
                    </p>
                    <a href=button_href class="inline-block transform transition-transform hover:scale-105">
                        <Button variant=ButtonVariant::Primary class="text-lg px-10 py-4 shadow-lg">
                            {button_text}
                        </Button>
                    </a>
                </div>
            </HeroContent>
        </Hero>
    }
}
