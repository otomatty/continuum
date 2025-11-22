use crate::components::{
    button::{Button, ButtonVariant},
    input::{Input, InputVariant},
    textarea::{Textarea, TextareaVariant},
};
use leptos::ev::InputEvent;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;

/**
 * ProfileSettingsForm Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/settings/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   ├─ app/src/components/input.rs
 *   ├─ app/src/components/textarea.rs
 *   └─ app/src/components/button.rs
 */
#[component]
pub fn ProfileSettingsForm(
    #[prop(optional)] on_save: Option<Callback<ProfileData>>,
) -> impl IntoView {
    let (display_name, set_display_name) = signal(String::new());
    let (avatar_url, set_avatar_url) = signal(String::new());
    let (bio, set_bio) = signal(String::new());
    let (twitter_url, set_twitter_url) = signal(String::new());
    let (linkedin_url, set_linkedin_url) = signal(String::new());

    let handle_display_name_input = move |ev: InputEvent| {
        let value = ev
            .target()
            .unwrap()
            .dyn_into::<leptos::web_sys::HtmlInputElement>()
            .unwrap()
            .value();
        set_display_name.set(value);
    };

    let handle_avatar_input = move |ev: InputEvent| {
        let value = ev
            .target()
            .unwrap()
            .dyn_into::<leptos::web_sys::HtmlInputElement>()
            .unwrap()
            .value();
        set_avatar_url.set(value);
    };

    let handle_bio_input = move |ev: InputEvent| {
        let value = ev
            .target()
            .unwrap()
            .dyn_into::<leptos::web_sys::HtmlTextAreaElement>()
            .unwrap()
            .value();
        set_bio.set(value);
    };

    let handle_twitter_input = move |ev: InputEvent| {
        let value = ev
            .target()
            .unwrap()
            .dyn_into::<leptos::web_sys::HtmlInputElement>()
            .unwrap()
            .value();
        set_twitter_url.set(value);
    };

    let handle_linkedin_input = move |ev: InputEvent| {
        let value = ev
            .target()
            .unwrap()
            .dyn_into::<leptos::web_sys::HtmlInputElement>()
            .unwrap()
            .value();
        set_linkedin_url.set(value);
    };

    let handle_save = move |_| {
        let data = ProfileData {
            display_name: display_name.get(),
            avatar_url: avatar_url.get(),
            bio: bio.get(),
            twitter_url: twitter_url.get(),
            linkedin_url: linkedin_url.get(),
        };
        if let Some(callback) = on_save.as_ref() {
            callback.run(data);
        }
    };

    view! {
        <div class="space-y-6">
            <h2 class="text-2xl font-bold">"Profile Settings"</h2>

            <div class="form-control w-full">
                <label class="label">
                    <span class="label-text">"Display Name"</span>
                </label>
                <Input
                    variant=InputVariant::Bordered
                    placeholder="Enter your display name".to_string()
                    value=display_name
                    on_input=Callback::new(handle_display_name_input)
                />
            </div>

            <div class="form-control w-full">
                <label class="label">
                    <span class="label-text">"Avatar URL"</span>
                </label>
                <Input
                    variant=InputVariant::Bordered
                    placeholder="https://example.com/avatar.png".to_string()
                    value=avatar_url
                    on_input=Callback::new(handle_avatar_input)
                />
                <label class="label">
                    <span class="label-text-alt">"Avatar is synced with GitHub by default"</span>
                </label>
            </div>

            <div class="form-control w-full">
                <label class="label">
                    <span class="label-text">"Bio"</span>
                </label>
                <Textarea
                    variant=TextareaVariant::Bordered
                    placeholder="Tell us about yourself".to_string()
                    rows=4
                    value=bio
                    on_input=Callback::new(handle_bio_input)
                />
            </div>

            <div class="form-control w-full">
                <label class="label">
                    <span class="label-text">"Twitter URL"</span>
                </label>
                <Input
                    variant=InputVariant::Bordered
                    placeholder="https://twitter.com/username".to_string()
                    value=twitter_url
                    on_input=Callback::new(handle_twitter_input)
                />
            </div>

            <div class="form-control w-full">
                <label class="label">
                    <span class="label-text">"LinkedIn URL"</span>
                </label>
                <Input
                    variant=InputVariant::Bordered
                    placeholder="https://linkedin.com/in/username".to_string()
                    value=linkedin_url
                    on_input=Callback::new(handle_linkedin_input)
                />
            </div>

            <div class="flex justify-end">
                <Button
                    variant=ButtonVariant::Primary
                    on_click=Callback::new(handle_save)
                >
                    "Save Changes"
                </Button>
            </div>
        </div>
    }
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct ProfileData {
    pub display_name: String,
    pub avatar_url: String,
    pub bio: String,
    pub twitter_url: String,
    pub linkedin_url: String,
}
