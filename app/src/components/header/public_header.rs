use super::theme_toggle::ThemeToggle;
use crate::components::container::Container;
use crate::components::github_login_button::GitHubLoginButton;
use leptos::prelude::*;

/**
 * PublicHeader Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/lib.rs
 *
 * Dependencies (External files that this component imports):
 *   ├─ app/src/components/github_login_button/mod.rs
 *   ├─ super::theme_toggle::ThemeToggle
 *   └─ crate::components::container::Container
 */
#[component]
pub fn PublicHeader() -> impl IntoView {
    view! {
        <header class="bg-base-100 shadow-sm sticky top-0 z-50 py-4">
            <Container class="max-w-full".to_string()>
                <div class="flex items-center justify-between w-full">
                    <h1 class="text-2xl font-bold">
                        <a href="/" class="btn btn-ghost text-2xl font-bold">
                            "Continuum"
                        </a>
                    </h1>
                    <div class="flex items-center gap-2">
                        <ThemeToggle />
                        <GitHubLoginButton />
                    </div>
                </div>
            </Container>
        </header>
    }
}
