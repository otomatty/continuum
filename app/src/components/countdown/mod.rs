/**
 * Countdown Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this Component):
 *   └─ (To be added when used)
 *
 * Dependencies (External files that this Component imports):
 *   └─ leptos::prelude
 *
 * Related Documentation:
 *   ├─ Spec: ./countdown.spec.md
 *   └─ Module: ../mod.rs
 */
use leptos::prelude::*;

#[cfg(feature = "hydrate")]
use js_sys::Date;

#[component]
pub fn Countdown(
    target_date: i64,
    #[prop(optional, into)] on_complete: Option<Callback<()>>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let (remaining_seconds, set_remaining_seconds) = signal(0i64);
    let (is_complete, set_is_complete) = signal(false);

    let update_countdown = move || {
        #[cfg(feature = "hydrate")]
        let now = Date::now() as i64 / 1000;
        #[cfg(not(feature = "hydrate"))]
        let now = 0i64;
        let remaining = target_date - now;

        if remaining <= 0 {
            set_remaining_seconds.set(0);
            set_is_complete.set(true);
            if let Some(callback) = on_complete.clone() {
                callback.run(());
            }
        } else {
            set_remaining_seconds.set(remaining);
        }
    };

    update_countdown();

    // set_interval_with_handleはブラウザ専用APIなので、クライアントサイドでのみ実行
    #[cfg(feature = "hydrate")]
    {
        let interval_handle = set_interval_with_handle(
            move || {
                update_countdown();
            },
            std::time::Duration::from_secs(1),
        )
        .ok();

        on_cleanup(move || {
            if let Some(handle) = interval_handle {
                handle.clear();
            }
        });
    }

    let countdown_class = if class.is_empty() {
        "countdown font-mono text-2xl".to_string()
    } else {
        format!("countdown font-mono text-2xl {}", class)
    };

    let hours = move || {
        let seconds = remaining_seconds.get();
        seconds / 3600
    };

    let minutes = move || {
        let seconds = remaining_seconds.get();
        (seconds % 3600) / 60
    };

    let seconds = move || {
        let secs = remaining_seconds.get();
        secs % 60
    };

    view! {
        <span class=countdown_class>
            <span
                style=move || format!("--value:{};", hours())
                aria-live="polite"
                aria-label=move || hours().to_string()
            >
                {move || format!("{}", hours())}
            </span>
            "h "
            <span
                style=move || format!("--value:{}; --digits: 2;", minutes())
                aria-live="polite"
                aria-label=move || minutes().to_string()
            >
                {move || format!("{:02}", minutes())}
            </span>
            "m "
            <span
                style=move || format!("--value:{}; --digits: 2;", seconds())
                aria-live="polite"
                aria-label=move || seconds().to_string()
            >
                {move || format!("{:02}", seconds())}
            </span>
            "s"
        </span>
    }
}
