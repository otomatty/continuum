/**
 * Pagination Component
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
 *   ├─ Spec: ./pagination.spec.md
 *   └─ Module: ../mod.rs
 */

use leptos::prelude::*;

#[component]
pub fn Pagination(
    current_page: usize,
    total_pages: usize,
    #[prop(optional, into)] on_page_change: Option<Callback<usize>>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let pagination_class = if class.is_empty() {
        "pagination".to_string()
    } else {
        format!("pagination {}", class)
    };

    let handle_page_change = move |page: usize| {
        if let Some(callback) = on_page_change.clone() {
            callback.run(page);
        }
    };

    let pages: Vec<usize> = (1..=total_pages).collect();
    let current = current_page;

    view! {
        <div class=pagination_class>
            <button
                class="btn"
                disabled=current == 1
                on:click=move |_| {
                    if current > 1 {
                        handle_page_change(current - 1);
                    }
                }
            >
                "«"
            </button>
            {pages.into_iter().map(move |page| {
                let is_active = page == current;
                let page_class = if is_active {
                    "btn btn-active"
                } else {
                    "btn"
                };
                let page_num = page;
                view! {
                    <button
                        class=page_class
                        on:click=move |_| handle_page_change(page_num)
                    >
                        {page_num.to_string()}
                    </button>
                }
            }).collect_view()}
            <button
                class="btn"
                disabled=current >= total_pages
                on:click=move |_| {
                    if current < total_pages {
                        handle_page_change(current + 1);
                    }
                }
            >
                "»"
            </button>
        </div>
    }
}

