use super::theme_toggle::ThemeToggle;
use crate::components::button::{Button, ButtonVariant};
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
 *   ├─ app/src/components/button.rs
 *   └─ super::theme_toggle::ThemeToggle
 */
#[component]
pub fn PublicHeader() -> impl IntoView {
    view! {
        <header class="navbar bg-base-100 shadow-sm sticky top-0 z-50">
            <div class="container mx-auto px-4">
                <div class="navbar-start">
                    <a href="/" class="btn btn-ghost text-xl font-bold">
                        "Continuum"
                    </a>
                </div>
                <div class="navbar-end gap-2">
                    <ThemeToggle />
                    <a href="/auth/login">
                        <Button variant=ButtonVariant::Primary class="btn-sm md:btn-md">
                            "GitHub でログイン"
                        </Button>
                    </a>
                </div>
            </div>
        </header>
    }
}
