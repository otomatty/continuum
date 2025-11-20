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

    let countdown_class = if class.is_empty() {
        "countdown".to_string()
    } else {
        format!("countdown {}", class)
    };

    let display_value = move || {
        if is_complete.get() {
            0
        } else {
            remaining_seconds.get()
        }
    };

    view! {
        <span class=countdown_class data-value=display_value>
            {move || {
                let seconds = remaining_seconds.get();
                let hours = seconds / 3600;
                let minutes = (seconds % 3600) / 60;
                let secs = seconds % 60;
                format!("{:02}:{:02}:{:02}", hours, minutes, secs)
            }}
        </span>
    }
}

