use super::contributor_card::ContributorCard;
use crate::concepts::user::User;
use leptos::prelude::*;

/**
 * ContributorGrid Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/contributors/components/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   ├─ app/src/concepts/user/mod.rs
 *   └─ app/src/pages/contributors/components/contributor_card.rs
 */
#[component]
pub fn ContributorGrid(users: Vec<User>, on_user_click: Option<Callback<String>>) -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {users.into_iter().map(move |user| {
                view! {
                    <ContributorCard
                        user=user.clone()
                        on_click=on_user_click
                    />
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
