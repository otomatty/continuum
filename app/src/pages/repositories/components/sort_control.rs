use crate::concepts::filter::{SortDirection, SortOption};
use leptos::prelude::*;

/**
 * SortControl Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/repositories/components/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   └─ app/src/concepts/filter/mod.rs
 */
#[component]
pub fn SortControl(
    sort_by: SortOption,
    direction: SortDirection,
    on_change: Callback<(SortOption, SortDirection)>,
) -> impl IntoView {
    let options = vec![
        (SortOption::UpdatedAt, "更新日"),
        (SortOption::Stars, "Star数"),
        (SortOption::Name, "名前"),
    ];

    let sort_by_clone = sort_by.clone();
    let direction_for_button = direction.clone();
    let direction_for_select = direction.clone();

    view! {
        <div class="flex items-center gap-2">
            <select
                class="select select-bordered select-sm"
                on:change=move |ev| {
                    let value = event_target_value(&ev);
                    let new_sort = match value.as_str() {
                        "updated" => SortOption::UpdatedAt,
                        "stars" => SortOption::Stars,
                        "name" => SortOption::Name,
                        _ => SortOption::UpdatedAt,
                    };
                    on_change.run((new_sort, direction_for_select.clone()));
                }
            >
                {options
                    .into_iter()
                    .map(|(opt, label)| {
                        let value = match opt {
                            SortOption::UpdatedAt => "updated",
                            SortOption::Stars => "stars",
                            SortOption::Name => "name",
                            _ => "updated",
                        };
                        let is_selected = std::mem::discriminant(&sort_by_clone) == std::mem::discriminant(&opt);
                        view! {
                            <option value=value selected=is_selected>
                                {label}
                            </option>
                        }
                    })
                    .collect_view()}
            </select>

            <button
                class="btn btn-sm btn-ghost"
                on:click=move |_| {
                    let new_direction = match direction {
                        SortDirection::Ascending => SortDirection::Descending,
                        SortDirection::Descending => SortDirection::Ascending,
                    };
                    on_change.run((sort_by.clone(), new_direction));
                }
            >
                {move || {
                    match direction_for_button {
                        SortDirection::Descending => "↓",
                        SortDirection::Ascending => "↑",
                    }
                }}
            </button>
        </div>
    }
}
