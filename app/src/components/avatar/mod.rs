use leptos::prelude::*;

#[component]
pub fn Avatar(
    src: String,
    #[prop(optional)] alt: String,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let avatar_class = if class.is_empty() {
        "avatar".to_string()
    } else {
        format!("avatar {}", class)
    };

    view! {
        <div class=avatar_class>
            <div class="w-12 rounded-full">
                <img src=src alt=alt />
            </div>
        </div>
    }
}

#[component]
pub fn AvatarGroup(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let group_class = if class.is_empty() {
        "avatar-group -space-x-6".to_string()
    } else {
        format!("avatar-group -space-x-6 {}", class)
    };

    view! {
        <div class=group_class>
            {children()}
        </div>
    }
}
