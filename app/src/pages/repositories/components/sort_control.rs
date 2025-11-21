use crate::components::select::{Select, SelectOption};
use leptos::prelude::*;

/**
 * SortOption enum for repository sorting
 */
#[derive(Debug, Clone, PartialEq)]
pub enum SortOption {
    NameAsc,
    NameDesc,
    StarsAsc,
    StarsDesc,
    UpdatedAsc,
    UpdatedDesc,
}

impl SortOption {
    pub fn as_string(&self) -> String {
        match self {
            SortOption::NameAsc => "name-asc".to_string(),
            SortOption::NameDesc => "name-desc".to_string(),
            SortOption::StarsAsc => "stars-asc".to_string(),
            SortOption::StarsDesc => "stars-desc".to_string(),
            SortOption::UpdatedAsc => "updated-asc".to_string(),
            SortOption::UpdatedDesc => "updated-desc".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "name-asc" => Some(SortOption::NameAsc),
            "name-desc" => Some(SortOption::NameDesc),
            "stars-asc" => Some(SortOption::StarsAsc),
            "stars-desc" => Some(SortOption::StarsDesc),
            "updated-asc" => Some(SortOption::UpdatedAsc),
            "updated-desc" => Some(SortOption::UpdatedDesc),
            _ => None,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            SortOption::NameAsc => "Name (A-Z)",
            SortOption::NameDesc => "Name (Z-A)",
            SortOption::StarsAsc => "Stars (Low to High)",
            SortOption::StarsDesc => "Stars (High to Low)",
            SortOption::UpdatedAsc => "Updated (Oldest)",
            SortOption::UpdatedDesc => "Updated (Newest)",
        }
    }
}

/**
 * SortControl Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/repositories/components/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   └─ app/src/components/select.rs
 */
#[component]
pub fn SortControl(
    sort_by: RwSignal<SortOption>,
    on_change: Callback<SortOption>,
) -> impl IntoView {
    let handle_change = move |value: String| {
        if let Some(sort_option) = SortOption::from_string(&value) {
            sort_by.set(sort_option.clone());
            on_change(sort_option);
        }
    };

    view! {
        <div class="flex items-center gap-2">
            <label class="text-sm font-medium">"Sort by:"</label>
            <Select
                value=sort_by.get().as_string()
                on_change=Callback::new(move |ev: leptos::ev::Event| {
                    handle_change(event_target_value(&ev));
                })
            >
                <SelectOption value=SortOption::NameAsc.as_string()>{SortOption::NameAsc.label()}</SelectOption>
                <SelectOption value=SortOption::NameDesc.as_string()>{SortOption::NameDesc.label()}</SelectOption>
                <SelectOption value=SortOption::StarsAsc.as_string()>{SortOption::StarsAsc.label()}</SelectOption>
                <SelectOption value=SortOption::StarsDesc.as_string()>{SortOption::StarsDesc.label()}</SelectOption>
                <SelectOption value=SortOption::UpdatedAsc.as_string()>{SortOption::UpdatedAsc.label()}</SelectOption>
                <SelectOption value=SortOption::UpdatedDesc.as_string()>{SortOption::UpdatedDesc.label()}</SelectOption>
            </Select>
        </div>
    }
}
