pub mod github;
pub mod components;
pub mod pages;
// pub mod mock; // Removed: mock module not found
pub mod concepts;
pub mod synchronizations;

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use components::navbar::Navbar;
use pages::{HomePage, DashboardPage, PortfolioPage};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/continuum.css"/>

        // sets the document title
        <Title text="Continuum - Portfolio Dashboard"/>

        // content for this welcome page
        <Router>
            <Navbar/>
            <main class="container mx-auto py-8">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("dashboard") view=DashboardPage/>
                    <Route path=StaticSegment("portfolio") view=PortfolioPage/>
                </Routes>
            </main>
        </Router>
    }
}
