use crate::components::{
    fade_in::FadeIn,
    stats::{StatDescription, StatItem, StatTitle, StatValue, Stats},
};
use leptos::prelude::*;

/**
 * StatisticsPreview Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/home/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   └─ app/src/components/stats.rs
 */
#[component]
pub fn StatisticsPreview(
    total_contributors: u32,
    total_repositories: u32,
    external_prs_this_month: u32,
) -> impl IntoView {
    let (contributors_display, set_contributors_display) = signal(0u32);
    let (repositories_display, set_repositories_display) = signal(0u32);
    let (prs_display, set_prs_display) = signal(0u32);

    #[cfg(feature = "hydrate")]
    {
        use wasm_bindgen_futures::spawn_local;
        use web_sys::window;

        Effect::new(move |_| {
            spawn_local(async move {
                let duration_ms = 2000u32; // 2 seconds
                let steps = 60u32;
                let interval_ms = duration_ms / steps;
                let contributors_step = total_contributors as f64 / steps as f64;
                let repositories_step = total_repositories as f64 / steps as f64;
                let prs_step = external_prs_this_month as f64 / steps as f64;

                for i in 0..=steps {
                    let progress = i as f64 / steps as f64;
                    let ease_out = 1.0 - (1.0 - progress).powf(3.0); // ease-out cubic

                    set_contributors_display
                        .set((contributors_step * steps as f64 * ease_out) as u32);
                    set_repositories_display
                        .set((repositories_step * steps as f64 * ease_out) as u32);
                    set_prs_display.set((prs_step * steps as f64 * ease_out) as u32);

                    if i < steps {
                        let window = window().unwrap();
                        let promise = js_sys::Promise::new(&mut |resolve, _| {
                            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                                &resolve,
                                interval_ms as i32,
                            );
                        });
                        let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                    }
                }

                // Ensure final values are set
                set_contributors_display.set(total_contributors);
                set_repositories_display.set(total_repositories);
                set_prs_display.set(external_prs_this_month);
            });
        });
    }

    #[cfg(not(feature = "hydrate"))]
    {
        // SSR: show final values immediately
        set_contributors_display.set(total_contributors);
        set_repositories_display.set(total_repositories);
        set_prs_display.set(external_prs_this_month);
    }

    view! {
        <FadeIn>
            <Stats class="w-full">
                <StatItem>
                    <StatTitle>"総コントリビューター数"</StatTitle>
                    <StatValue>{move || contributors_display.get().to_string()}</StatValue>
                    <StatDescription>"組織内のアクティブなコントリビューター"</StatDescription>
                </StatItem>
                <StatItem>
                    <StatTitle>"アクティブなリポジトリ数"</StatTitle>
                    <StatValue>{move || repositories_display.get().to_string()}</StatValue>
                    <StatDescription>"最近活動のあるリポジトリ"</StatDescription>
                </StatItem>
                <StatItem>
                    <StatTitle>"外部からのPR数"</StatTitle>
                    <StatValue>{move || prs_display.get().to_string()}</StatValue>
                    <StatDescription>"今月"</StatDescription>
                </StatItem>
            </Stats>
        </FadeIn>
    }
}
