use crate::components::{
    checkbox::{Checkbox, CheckboxVariant},
    toggle::{Toggle, ToggleVariant},
};
use leptos::prelude::*;

/**
 * NotificationSettings Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/settings/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   ├─ app/src/components/toggle.rs
 *   └─ app/src/components/checkbox.rs
 */
#[component]
pub fn NotificationSettings(
    #[prop(optional)] on_change: Option<Callback<NotificationData>>,
) -> impl IntoView {
    let (email_notifications, set_email_notifications) = signal(true);
    let (notify_commits, set_notify_commits) = signal(true);
    let (notify_prs, set_notify_prs) = signal(true);
    let (notify_reviews, set_notify_reviews) = signal(false);
    let (notify_mentions, set_notify_mentions) = signal(true);

    let update_notifications = move || {
        if let Some(callback) = on_change.as_ref() {
            callback.run(NotificationData {
                email_notifications: email_notifications.get(),
                notify_commits: notify_commits.get(),
                notify_prs: notify_prs.get(),
                notify_reviews: notify_reviews.get(),
                notify_mentions: notify_mentions.get(),
            });
        }
    };

    let handle_email_toggle = move |_| {
        set_email_notifications.update(|v| *v = !*v);
        update_notifications();
    };

    let handle_commits_toggle = move |_| {
        set_notify_commits.update(|v| *v = !*v);
        update_notifications();
    };

    let handle_prs_toggle = move |_| {
        set_notify_prs.update(|v| *v = !*v);
        update_notifications();
    };

    let handle_reviews_toggle = move |_| {
        set_notify_reviews.update(|v| *v = !*v);
        update_notifications();
    };

    let handle_mentions_toggle = move |_| {
        set_notify_mentions.update(|v| *v = !*v);
        update_notifications();
    };

    view! {
        <div class="space-y-6">
            <h2 class="text-2xl font-bold">"Notification Settings"</h2>

            <div class="form-control">
                <label class="label cursor-pointer">
                    <span class="label-text font-semibold">"Email Notifications"</span>
                    <Toggle
                        variant=ToggleVariant::Primary
                        checked=email_notifications
                        on_change=Callback::new(handle_email_toggle)
                    />
                </label>
                <label class="label">
                    <span class="label-text-alt">"Receive email notifications for activities"</span>
                </label>
            </div>

            <div class="divider"></div>

            <div class="space-y-4">
                <h3 class="text-lg font-semibold">"Notify me about:"</h3>

                <label class="label cursor-pointer">
                    <span class="label-text">"Commits"</span>
                    <Checkbox
                        variant=CheckboxVariant::Primary
                        checked=notify_commits
                        on_change=Callback::new(handle_commits_toggle)
                    />
                </label>

                <label class="label cursor-pointer">
                    <span class="label-text">"Pull Requests"</span>
                    <Checkbox
                        variant=CheckboxVariant::Primary
                        checked=notify_prs
                        on_change=Callback::new(handle_prs_toggle)
                    />
                </label>

                <label class="label cursor-pointer">
                    <span class="label-text">"Reviews"</span>
                    <Checkbox
                        variant=CheckboxVariant::Primary
                        checked=notify_reviews
                        on_change=Callback::new(handle_reviews_toggle)
                    />
                </label>

                <label class="label cursor-pointer">
                    <span class="label-text">"Mentions"</span>
                    <Checkbox
                        variant=CheckboxVariant::Primary
                        checked=notify_mentions
                        on_change=Callback::new(handle_mentions_toggle)
                    />
                </label>
            </div>
        </div>
    }
}

#[derive(Clone)]
pub struct NotificationData {
    pub email_notifications: bool,
    pub notify_commits: bool,
    pub notify_prs: bool,
    pub notify_reviews: bool,
    pub notify_mentions: bool,
}
