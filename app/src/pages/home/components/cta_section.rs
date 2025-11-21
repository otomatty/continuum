use leptos::prelude::*;
use crate::components::{
    button::{Button, ButtonVariant},
    hero::{Hero, HeroContent},
};

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
        <Hero class="bg-base-200 py-16">
            <HeroContent class="text-center">
                <div class="max-w-2xl">
                    <h1 class="text-5xl font-bold mb-4">{headline}</h1>
                    <p class="text-xl text-gray-600 mb-8">{subheadline}</p>
                    <a href=button_href>
                        <Button variant=ButtonVariant::Primary class="text-lg px-8 py-3">
                            {button_text}
                        </Button>
                    </a>
                </div>
            </HeroContent>
        </Hero>
    }
}

