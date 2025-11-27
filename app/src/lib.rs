pub mod components;
pub mod github;
pub mod pages;
// pub mod mock; // Removed: mock module not found
pub mod concepts;
pub mod hooks;
pub mod synchronizations;

use components::header::{AuthenticatedHeader, PublicHeader};
use concepts::theme::{Theme, ThemeState};
use hooks::AuthStatus;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    hooks::use_location,
    path, StaticSegment,
};
use pages::{
    AuthErrorPage, ComponentsPage, ContributorsPage, DashboardPage, HomePage, KnowledgePage,
    NotFoundPage, PortfolioPage, RepositoriesPage, SettingsPage,
};

// web_sys::window is only available in WASM targets
#[cfg(target_arch = "wasm32")]
use web_sys::window;

#[cfg(target_arch = "wasm32")]
const THEME_STORAGE_KEY: &str = "continuum_v1_theme";

#[cfg(target_arch = "wasm32")]
fn get_theme_from_storage() -> Option<Theme> {
    let window = window()?;
    let storage = window.local_storage().ok()??;
    let theme_str = storage.get_item(THEME_STORAGE_KEY).ok()??;
    match theme_str.as_str() {
        "light" => Some(Theme::Light),
        "dark" => Some(Theme::Dark),
        "system" => Some(Theme::System),
        _ => None,
    }
}

#[cfg(target_arch = "wasm32")]
fn save_theme_to_storage(theme: Theme) {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let theme_str = match theme {
                Theme::Light => "light",
                Theme::Dark => "dark",
                Theme::System => "system",
            };
            let _ = storage.set_item(THEME_STORAGE_KEY, theme_str);
        }
    }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    // Get auth status from cookies (SSR only)
    // Note: In Leptos shell function, we cannot directly access cookies synchronously
    // The auth status will be retrieved client-side from the data attribute
    // For SSR, we'll use a placeholder that will be replaced by client-side code
    #[cfg(feature = "ssr")]
    let auth_status = {
        // Try to get auth status from the request context
        // Since we can't directly access cookies in shell function,
        // we'll default to unauthenticated and let the client-side code
        // fetch the actual auth status after hydration
        // The actual auth status will be set via Server Function on client-side
        AuthStatus {
            authenticated: false,
            user_id: None,
        }
    };

    #[cfg(not(feature = "ssr"))]
    let auth_status = AuthStatus {
        authenticated: false,
        user_id: None,
    };

    // Serialize auth status to JSON for embedding in HTML
    let auth_status_json = serde_json::to_string(&auth_status).unwrap_or_else(|_| {
        serde_json::to_string(&AuthStatus {
            authenticated: false,
            user_id: None,
        })
        .unwrap()
    });

    view! {
        <!DOCTYPE html>
        <html lang="en" data-auth-status=auth_status_json>
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

    // Initialize auth context - read from HTML data attribute on client side
    #[cfg(target_arch = "wasm32")]
    let initial_auth_status = {
        use crate::hooks::read_auth_status_from_html;
        read_auth_status_from_html()
    };

    #[cfg(not(target_arch = "wasm32"))]
    let initial_auth_status = None::<crate::hooks::AuthStatus>;

    let auth_context = crate::hooks::provide_auth_context(initial_auth_status);

    // Refresh auth status from server on page load (client-side only)
    // This ensures the auth status is up-to-date after page reload
    #[cfg(target_arch = "wasm32")]
    {
        use crate::hooks::refresh_auth_status_if_needed;
        use leptos::task::spawn_local;

        let context_for_effect = auth_context.clone();
        Effect::new(move |_| {
            let context = context_for_effect.clone();
            spawn_local(async move {
                let _ = refresh_auth_status_if_needed(&context).await;
            });
        });
    }

    // Suppress unused variable warning in non-wasm32 builds
    #[cfg(not(target_arch = "wasm32"))]
    let _ = auth_context;

    // Initialize theme state - load from localStorage if available (client-side only)
    let initial_theme_state = {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(stored_theme) = get_theme_from_storage() {
                ThemeState {
                    current_theme: stored_theme,
                }
            } else {
                ThemeState::default()
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            ThemeState::default()
        }
    };
    let (theme_state, set_theme_state) = signal(initial_theme_state);

    // Save theme to localStorage when it changes (client-side only)
    #[cfg(target_arch = "wasm32")]
    {
        Effect::new(move |_| {
            let state = theme_state.get();
            save_theme_to_storage(state.current_theme);
        });
    }

    // Provide theme context to all child components
    provide_context(theme_state);
    provide_context(set_theme_state);

    // Get effective theme for data-theme attribute
    let effective_theme = move || {
        let state = theme_state.get();
        match state.current_theme {
            Theme::Light => "light",
            Theme::Dark => "dark",
            Theme::System => {
                #[cfg(target_arch = "wasm32")]
                {
                    // Check system preference
                    if let Some(window) = window() {
                        if let Ok(Some(media_query)) =
                            window.match_media("(prefers-color-scheme: dark)")
                        {
                            if media_query.matches() {
                                return "dark";
                            }
                        }
                    }
                }
                "light" // Default to light
            }
        }
    };

    view! {
        <Stylesheet id="leptos" href="/pkg/continuum.css"/>

        // sets the document title
        <Title text="Continuum - Portfolio Dashboard"/>

        // content for this welcome page
        <div data-theme=effective_theme>
            <Router>
                <AppHeader />
                <MainContent />
            </Router>
        </div>
    }
}

#[component]
fn AppHeader() -> impl IntoView {
    let location = use_location();
    let path = move || location.pathname.get();

    // 認証が必要なページのリスト
    let authenticated_paths = [
        "/dashboard",
        "/portfolio",
        "/knowledge",
        "/contributors",
        "/repositories",
        "/settings",
    ];

    let is_authenticated = move || {
        let current_path = path();
        authenticated_paths
            .iter()
            .any(|p| current_path.starts_with(p))
    };

    view! {
        <Show when=is_authenticated fallback=|| view! { <PublicHeader /> }>
            <AuthenticatedHeader />
        </Show>
    }
}

#[component]
fn MainContent() -> impl IntoView {
    let location = use_location();
    let current_path = move || location.pathname.get();
    let main_class = move || {
        if current_path() == "/" {
            "".to_string()
        } else {
            "container mx-auto py-8".to_string()
        }
    };

    view! {
        <main class=main_class>
            <Routes fallback=|| view! { <NotFoundPage /> }>
                <Route path=StaticSegment("") view=HomePage/>
                <Route path=StaticSegment("dashboard") view=DashboardPage/>
                <Route path=StaticSegment("portfolio") view=PortfolioPage/>
                <Route path=path!("/portfolio/:username") view=PortfolioPage/>
                <Route path=StaticSegment("components") view=ComponentsPage/>
                <Route path=StaticSegment("contributors") view=ContributorsPage/>
                <Route path=StaticSegment("repositories") view=RepositoriesPage/>
                <Route path=StaticSegment("knowledge") view=KnowledgePage/>
                <Route path=StaticSegment("settings") view=SettingsPage/>
                <Route path=path!("/auth/error") view=AuthErrorPage/>
            </Routes>
        </main>
    }
}
