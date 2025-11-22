use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="navbar">
            <div class="navbar-start">
                <a href="/" class="btn btn-ghost text-xl font-bold">
                    "Continuum"
                </a>
            </div>
            <div class="navbar-end">
                <a href="/" class="btn btn-ghost">
                    "Home"
                </a>
                <a href="/dashboard" class="btn btn-ghost">
                    "Dashboard"
                </a>
                <a href="/portfolio/alice-dev" class="btn btn-ghost">
                    "Portfolio"
                </a>
                <a href="/components" class="btn btn-ghost">
                    "Components"
                </a>
            </div>
        </nav>
    }
}
