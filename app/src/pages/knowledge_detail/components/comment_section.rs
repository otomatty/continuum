use crate::components::{
    avatar::Avatar,
    button::Button,
    button::ButtonVariant,
    card::{Card, CardBody},
};
use leptos::prelude::*;

/**
 * CommentSection Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/knowledge_detail/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   ├─ app/src/components/card.rs
 *   ├─ app/src/components/avatar.rs
 *   └─ app/src/components/button.rs
 */
#[derive(Clone)]
pub struct Comment {
    pub id: String,
    pub author_name: String,
    pub author_avatar: String,
    pub author_username: String,
    pub content: String,
    pub created_at: String,
    pub reactions: Vec<(String, u32)>, // (emoji, count)
    pub replies: Vec<Comment>,
}

#[component]
pub fn CommentSection(
    comments: Vec<Comment>,
    #[prop(optional)] on_reply: Option<Callback<String>>,
) -> impl IntoView {
    view! {
        <div class="space-y-4">
            <h3 class="text-2xl font-bold mb-4">"Comments"</h3>
            {comments.into_iter().map(|comment| {
                let on_reply_clone = on_reply;
                if let Some(callback) = on_reply_clone {
                    view! {
                        <CommentItem
                            comment=comment
                            on_reply=callback
                            depth=0usize
                        />
                    }
                } else {
                    view! {
                        <CommentItem
                            comment=comment
                            depth=0usize
                        />
                    }
                }
            }).collect_view()}
        </div>
    }
}

#[component]
fn CommentItem(
    comment: Comment,
    #[prop(optional)] on_reply: Option<Callback<String>>,
    depth: usize,
) -> impl IntoView {
    let handle_reply = move |_| {
        if let Some(callback) = on_reply {
            callback.run(comment.id.clone());
        }
    };

    let margin_left = format!("ml-{}", depth * 8);

    view! {
        <Card class=format!("{}", margin_left)>
            <CardBody>
                <div class="flex gap-4">
                    <Avatar
                        src=comment.author_avatar.clone()
                        alt=comment.author_name.clone()
                        class="w-10 h-10"
                    />
                    <div class="flex-1">
                        <div class="flex items-center gap-2 mb-2">
                            <span class="font-semibold">{comment.author_name.clone()}</span>
                            <span class="text-sm text-gray-500">"@" {comment.author_username.clone()}</span>
                            <span class="text-sm text-gray-400">{comment.created_at.clone()}</span>
                        </div>
                        <div class="prose max-w-none mb-3" inner_html=comment.content.clone() />
                        <div class="flex items-center gap-4">
                            {comment.reactions.iter().map(|(emoji, count)| {
                                let emoji_clone = emoji.clone();
                                let count_clone = *count;
                                view! {
                                    <span class="text-sm text-gray-600">
                                        {emoji_clone} " " {count_clone}
                                    </span>
                                }
                            }).collect_view()}
                            <Button
                                variant=ButtonVariant::Ghost
                                class="text-sm".to_string()
                                on_click=Callback::new(handle_reply)
                            >
                                "Reply"
                            </Button>
                        </div>
                        {if !comment.replies.is_empty() {
                            let replies = comment.replies.clone();
                            let on_reply_clone = on_reply;
                            Some(view! {
                                <div class="mt-4 space-y-2">
                                    {replies.into_iter().map(|reply| {
                                        let on_reply_clone2 = on_reply_clone;
                                        if let Some(callback) = on_reply_clone2 {
                                            view! {
                                                <CommentItem
                                                    comment=reply
                                                    on_reply=callback
                                                    depth=depth + 1
                                                />
                                            }
                                        } else {
                                            view! {
                                                <CommentItem
                                                    comment=reply
                                                    depth=depth + 1
                                                />
                                            }
                                        }
                                    }).collect_view()}
                                </div>
                            })
                        } else {
                            None
                        }}
                    </div>
                </div>
            </CardBody>
        </Card>
    }
}
