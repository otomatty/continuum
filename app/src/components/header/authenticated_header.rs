use super::theme_toggle::ThemeToggle;
use crate::components::container::Container;
use leptos::prelude::*;

/**
 * AuthenticatedHeader Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/lib.rs
 *
 * Dependencies (External files that this component imports):
 *   ├─ leptos::prelude
 *   ├─ super::theme_toggle::ThemeToggle
 *   └─ crate::components::container::Container
 */
#[component]
pub fn AuthenticatedHeader() -> impl IntoView {
    let (dropdown_open, set_dropdown_open) = signal(false);

    view! {
        <header class="bg-base-100 shadow-sm sticky top-0 z-50">
            <Container class="max-w-full".to_string()>
                <div class="flex items-center justify-between">
                    <a href="/dashboard" class="btn btn-ghost text-2xl font-bold">
                        "Continuum"
                    </a>
                </div>
                <div class="navbar-center hidden lg:flex">
                    <ul class="menu menu-horizontal px-1 gap-1">
                        <li>
                            <a href="/dashboard" class="btn btn-ghost btn-sm">
                                "ダッシュボード"
                            </a>
                        </li>
                        <li>
                            <a href="/knowledge" class="btn btn-ghost btn-sm">
                                "ナレッジ"
                            </a>
                        </li>
                        <li>
                            <a href="/contributors" class="btn btn-ghost btn-sm">
                                "コントリビューター"
                            </a>
                        </li>
                        <li>
                            <a href="/repositories" class="btn btn-ghost btn-sm">
                                "リポジトリ"
                            </a>
                        </li>
                    </ul>
                </div>
                <div class="navbar-end gap-2">
                    // Theme toggle
                    <ThemeToggle />

                    // Mobile menu button
                    <div class="dropdown dropdown-end lg:hidden">
                        <div
                            tabindex="0"
                            role="button"
                            class="btn btn-ghost btn-circle"
                            on:click=move |_| set_dropdown_open.set(!dropdown_open.get())
                        >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="h-5 w-5"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M4 6h16M4 12h16M4 18h16"
                                />
                            </svg>
                        </div>
                        <ul
                            tabindex="0"
                            class=move || {
                                if dropdown_open.get() {
                                    "menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52"
                                } else {
                                    "menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52 hidden"
                                }
                            }
                        >
                            <li><a href="/dashboard">"ダッシュボード"</a></li>
                            <li><a href="/knowledge">"ナレッジ"</a></li>
                            <li><a href="/contributors">"コントリビューター"</a></li>
                            <li><a href="/repositories">"リポジトリ"</a></li>
                            <li class="divider"></li>
                            <li><a href="/portfolio">"マイポートフォリオ"</a></li>
                            <li><a href="/settings">"設定"</a></li>
                            <li><a href="/auth/logout">"ログアウト"</a></li>
                        </ul>
                    </div>

                    // Desktop user dropdown
                    <div class="dropdown dropdown-end hidden lg:block">
                        <div tabindex="0" role="button" class="btn btn-ghost btn-circle avatar">
                            <div class="w-10 rounded-full bg-primary text-primary-content flex items-center justify-center">
                                <span class="text-sm font-semibold">"U"</span>
                            </div>
                        </div>
                        <ul
                            tabindex="0"
                            class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52"
                        >
                            <li><a href="/portfolio">"マイポートフォリオ"</a></li>
                            <li><a href="/settings">"設定"</a></li>
                            <li class="divider"></li>
                            <li><a href="/auth/logout">"ログアウト"</a></li>
                        </ul>
                    </div>
                </div>
            </Container>
        </header>
    }
}
