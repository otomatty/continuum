/**
 * Not Found (404) Page
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this page):
 *   └─ app/src/pages/mod.rs
 *
 * Dependencies (External files that this page imports):
 *   ├─ leptos::prelude
 *   └─ crate::components::container::Container
 *
 * Related Documentation:
 *   └─ Implementation Roadmap: docs/03_plans/continuum/20251121_implementation-roadmap.md
 */
use crate::components::container::Container;
use leptos::prelude::*;

/// 404 Not Found ページ
#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <Container>
            <div class="min-h-[60vh] flex items-center justify-center">
                <div class="text-center">
                    // 404 illustration
                    <div class="mb-8">
                        <div class="text-9xl font-bold text-primary/20">
                            "404"
                        </div>
                    </div>

                    // Title
                    <h1 class="text-3xl font-bold mb-4">
                        "ページが見つかりません"
                    </h1>

                    // Description
                    <p class="text-base-content/70 mb-8 max-w-md mx-auto">
                        "お探しのページは存在しないか、移動した可能性があります。URLをご確認ください。"
                    </p>

                    // Action buttons
                    <div class="flex flex-col sm:flex-row gap-4 justify-center">
                        <a href="/" class="btn btn-primary">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
                            </svg>
                            "ホームに戻る"
                        </a>
                        <a href="/dashboard" class="btn btn-ghost">
                            "ダッシュボードへ"
                        </a>
                    </div>
                </div>
            </div>
        </Container>
    }
}
