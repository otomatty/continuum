use leptos::prelude::*;
use crate::components::radio::Radio;
use crate::concepts::user::UserRole;

/**
 * StatusFilter Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/contributors/components/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   ├─ app/src/components/radio.rs
 *   └─ app/src/concepts/user/mod.rs
 */
#[component]
pub fn StatusFilter(
    selected_status: RwSignal<Option<UserRole>>,
    on_change: Callback<Option<UserRole>>,
) -> impl IntoView {
    let handle_change = move |role: Option<UserRole>| {
        selected_status.set(role.clone());
        on_change.call(role);
    };

    let current_checked = move || selected_status.get() == Some(UserRole::CurrentEmployee);
    let alumni_checked = move || selected_status.get() == Some(UserRole::Alumni);
    let external_checked = move || selected_status.get() == Some(UserRole::ExternalContributor);
    let all_checked = move || selected_status.get().is_none();

    view! {
        <div class="flex flex-wrap gap-4 p-4 bg-base-200 rounded-lg">
            <label class="flex items-center gap-2 cursor-pointer">
                <Radio
                    name="status-filter".to_string()
                    value="all".to_string()
                    checked=all_checked()
                    on_change=Callback::new(move |_| handle_change(None))
                />
                <span class="text-sm font-medium">"All"</span>
            </label>
            <label class="flex items-center gap-2 cursor-pointer">
                <Radio
                    name="status-filter".to_string()
                    value="current".to_string()
                    checked=current_checked()
                    on_change=Callback::new(move |_| handle_change(Some(UserRole::CurrentEmployee)))
                />
                <span class="text-sm font-medium">"Current"</span>
            </label>
            <label class="flex items-center gap-2 cursor-pointer">
                <Radio
                    name="status-filter".to_string()
                    value="alumni".to_string()
                    checked=alumni_checked()
                    on_change=Callback::new(move |_| handle_change(Some(UserRole::Alumni)))
                />
                <span class="text-sm font-medium">"Alumni"</span>
            </label>
            <label class="flex items-center gap-2 cursor-pointer">
                <Radio
                    name="status-filter".to_string()
                    value="external".to_string()
                    checked=external_checked()
                    on_change=Callback::new(move |_| handle_change(Some(UserRole::ExternalContributor)))
                />
                <span class="text-sm font-medium">"External"</span>
            </label>
        </div>
    }
}

