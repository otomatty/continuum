use crate::components::container::Container;
use leptos::prelude::*;

/**
 * HomeFooter Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/home/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   ├─ leptos::prelude
 *   └─ app/src/components/container.rs
 */
#[component]
pub fn HomeFooter() -> impl IntoView {
    view! {
        <footer class="footer footer-center bg-base-200 text-base-content p-10">
            <Container>
                <div class="grid grid-flow-col gap-4">
                    <a class="link link-hover" href="/dashboard">"ダッシュボード"</a>
                    <a class="link link-hover" href="/knowledge">"ナレッジ"</a>
                    <a class="link link-hover" href="/contributors">"コントリビューター"</a>
                    <a class="link link-hover" href="/repositories">"リポジトリ"</a>
                </div>
                <div class="grid grid-flow-col gap-4 mt-4">
                    <a class="link link-hover" href="/about">"About"</a>
                    <a class="link link-hover" href="/contact">"お問い合わせ"</a>
                </div>
                <div class="mt-6">
                    <p class="text-sm text-gray-500">
                        "© 2025 Continuum. All rights reserved."
                    </p>
                    <p class="text-xs text-gray-400 mt-2">
                        "エンジニアの成長を、組織全体で支援する"
                    </p>
                </div>
            </Container>
        </footer>
    }
}
