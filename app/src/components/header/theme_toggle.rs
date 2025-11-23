use crate::concepts::theme::{set_theme, Theme, ThemeState};
use leptos::prelude::*;

/**
 * ThemeToggle Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   ├─ app/src/components/header/authenticated_header.rs
 *   └─ app/src/components/header/public_header.rs
 *
 * Dependencies (External files that this component imports):
 *   ├─ leptos::prelude
 *   └─ crate::concepts::theme
 */
#[component]
pub fn ThemeToggle() -> impl IntoView {
    let theme_state = use_context::<ReadSignal<ThemeState>>()
        .expect("ThemeToggle must be used within App component");
    let set_theme_state = use_context::<WriteSignal<ThemeState>>()
        .expect("ThemeToggle must be used within App component");

    let (dropdown_open, set_dropdown_open) = signal(false);

    let current_theme = move || theme_state.get().current_theme;

    let dropdown_class = move || {
        if dropdown_open.get() {
            "dropdown dropdown-end dropdown-open"
        } else {
            "dropdown dropdown-end"
        }
    };

    let light_class = move || {
        if current_theme() == Theme::Light {
            "active"
        } else {
            ""
        }
    };
    let dark_class = move || {
        if current_theme() == Theme::Dark {
            "active"
        } else {
            ""
        }
    };
    let system_class = move || {
        if current_theme() == Theme::System {
            "active"
        } else {
            ""
        }
    };

    let handle_set_theme = move |theme: Theme| {
        let current = theme_state.get();
        let new_state = set_theme(current, theme);
        set_theme_state.set(new_state);
        set_dropdown_open.set(false);
    };

    let handle_toggle = move |_| {
        set_dropdown_open.set(!dropdown_open.get());
    };

    view! {
        <div class=dropdown_class>
            <div
                tabindex="0"
                role="button"
                class="p-2 rounded-full hover:bg-base-200 cursor-pointer transition-colors"
                on:click=handle_toggle
            >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-6 w-6"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"
                    />
                </svg>
            </div>
            <ul
                tabindex="0"
                class="dropdown-content menu bg-base-100 rounded-box z-[1] w-52 p-2 shadow"
            >
                <li>
                    <button
                        class=light_class
                        on:click=move |_| handle_set_theme(Theme::Light)
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-6 w-6"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"
                            />
                        </svg>
                        "ライト"
                    </button>
                </li>
                <li>
                    <button
                        class=dark_class
                        on:click=move |_| handle_set_theme(Theme::Dark)
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-6 w-6"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"
                            />
                        </svg>
                        "ダーク"
                    </button>
                </li>
                <li>
                    <button
                        class=system_class
                        on:click=move |_| handle_set_theme(Theme::System)
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-6 w-6"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"
                            />
                        </svg>
                        "システム"
                    </button>
                </li>
            </ul>
        </div>
    }
}
