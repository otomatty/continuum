use crate::components::{
    button::{Button, ButtonVariant},
    container::Container,
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
 *   ├─ app/src/components/button.rs
 *   └─ app/src/components/container.rs
 */
#[component]
pub fn FinalCTA() -> impl IntoView {
    view! {
        <section class="bg-gradient-to-br from-primary/10 to-secondary/10 py-16">
            <Container>
                <div class="max-w-2xl mx-auto text-center">
                    <h2 class="text-3xl font-bold mb-4">"今すぐ始めて、成長を加速させる"</h2>
                    <p class="text-lg text-gray-600 mb-8">
                        "社内エンジニア向けの成長支援ツール。GitHubアカウントがあれば、すぐに始められます。"
                    </p>
                    <a href="/auth/login">
                        <Button variant=ButtonVariant::Primary class="text-lg px-8 py-3">
                            "GitHub でログイン"
                        </Button>
                    </a>
                    <p class="text-sm text-gray-500 mt-4">
                        "組織のエンジニアが働きやすく成長しやすい環境を提供するための一環です"
                    </p>
                </div>
            </Container>
        </section>
    }
}
