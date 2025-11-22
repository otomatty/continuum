use crate::components::button::{Button, ButtonVariant};
use leptos::prelude::*;

/**
 * ReactionButtons Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/knowledge_detail/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   └─ app/src/components/button.rs
 */
#[derive(Clone)]
pub struct Reaction {
    pub emoji: String,
    pub count: u32,
    pub active: bool,
}

#[component]
pub fn ReactionButtons(
    reactions: Vec<Reaction>,
    #[prop(optional)] on_reaction: Option<Callback<String>>,
) -> impl IntoView {
    view! {
        <div class="flex items-center gap-2 flex-wrap">
            {reactions.into_iter().map(|reaction| {
                let emoji = reaction.emoji.clone();
                let emoji_clone = emoji.clone();
                let count = reaction.count;
                let active = reaction.active;
                let handle_click = move |_| {
                    if let Some(callback) = on_reaction {
                        callback.run(emoji_clone.clone());
                    }
                };

                view! {
                    <Button
                        variant=if active { ButtonVariant::Primary } else { ButtonVariant::Ghost }
                        class=format!("gap-2 {}", if active { "btn-active" } else { "" })
                        on_click=Callback::new(handle_click)
                    >
                        <span>{emoji}</span>
                        <span>{count}</span>
                    </Button>
                }
            }).collect_view()}
        </div>
    }
}
