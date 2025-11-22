/**
 * Rating Component
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
 *   ├─ Spec: ./rating.spec.md
 *   └─ Module: ../mod.rs
 */
use leptos::prelude::*;

#[component]
pub fn Rating(
    value: f64,
    #[prop(optional)] readonly: bool,
    #[prop(optional, into)] on_change: Option<Callback<f64>>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let rating_class = if class.is_empty() {
        "rating".to_string()
    } else {
        format!("rating {}", class)
    };

    let handle_click = move |index: usize| {
        if !readonly {
            let new_value = (index + 1) as f64;
            if let Some(callback) = on_change {
                callback.run(new_value);
            }
        }
    };

    let stars = (0..5)
        .map(move |index| {
            let star_value = (index + 1) as f64;
            let is_hidden = if readonly { star_value > value } else { false };

            let star_class = if is_hidden {
                "rating-hidden"
            } else if star_value <= value {
                ""
            } else if star_value - 0.5 <= value {
                "rating-half"
            } else {
                "rating-hidden"
            };

            view! {
                <input
                    type="radio"
                    name="rating"
                    class=format!("mask mask-star-2 {}", star_class)
                    checked=move || star_value <= value
                    disabled=readonly
                    on:click=move |_| handle_click(index)
                />
            }
        })
        .collect_view();

    view! {
        <div class=rating_class>
            {stars}
        </div>
    }
}
