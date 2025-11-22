use crate::components::{
    radio::{Radio, RadioVariant},
    toggle::{Toggle, ToggleVariant},
};
use leptos::ev::Event;
use leptos::prelude::*;

/**
 * PrivacySettings Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/settings/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   ├─ app/src/components/radio.rs
 *   └─ app/src/components/toggle.rs
 */
#[derive(Clone, Copy, PartialEq)]
pub enum PortfolioVisibility {
    Public,
    Private,
}

#[component]
pub fn PrivacySettings(
    #[prop(optional)] on_change: Option<Callback<PrivacyData>>,
) -> impl IntoView {
    let (portfolio_visibility, set_portfolio_visibility) = signal(PortfolioVisibility::Public);
    let (show_activity, set_show_activity) = signal(true);

    let (public_checked, set_public_checked) = signal(true);
    let (private_checked, set_private_checked) = signal(false);

    Effect::new(move |_| {
        set_public_checked.set(portfolio_visibility.get() == PortfolioVisibility::Public);
        set_private_checked.set(portfolio_visibility.get() == PortfolioVisibility::Private);
    });

    let handle_visibility_change = move |_| {
        // Radio button change handled by signal
        if let Some(callback) = on_change.as_ref() {
            callback.run(PrivacyData {
                portfolio_visibility: portfolio_visibility.get(),
                show_activity: show_activity.get(),
            });
        }
    };

    let handle_activity_toggle = move |_| {
        set_show_activity.update(|v| *v = !*v);
        if let Some(callback) = on_change.as_ref() {
            callback.run(PrivacyData {
                portfolio_visibility: portfolio_visibility.get(),
                show_activity: show_activity.get(),
            });
        }
    };

    view! {
        <div class="space-y-6">
            <h2 class="text-2xl font-bold">"Privacy Settings"</h2>

            <div class="form-control">
                <label class="label">
                    <span class="label-text font-semibold">"Portfolio Visibility"</span>
                </label>
                <div class="flex flex-col gap-2 mt-2">
                    <label class="label cursor-pointer">
                        <span class="label-text">"Public"</span>
                        <Radio
                            name="portfolio-visibility".to_string()
                            value="public".to_string()
                            variant=RadioVariant::Primary
                            checked=public_checked
                            on_change=Callback::new(handle_visibility_change)
                        />
                    </label>
                    <label class="label cursor-pointer">
                        <span class="label-text">"Private"</span>
                        <Radio
                            name="portfolio-visibility".to_string()
                            value="private".to_string()
                            variant=RadioVariant::Primary
                            checked=private_checked
                            on_change=Callback::new(move |_| {
                                set_portfolio_visibility.set(PortfolioVisibility::Private);
                                handle_visibility_change(Event::new("change").unwrap());
                            })
                        />
                    </label>
                </div>
            </div>

            <div class="form-control">
                <label class="label cursor-pointer">
                    <span class="label-text font-semibold">"Show Activity"</span>
                    <Toggle
                        variant=ToggleVariant::Primary
                        checked=show_activity
                        on_change=Callback::new(handle_activity_toggle)
                    />
                </label>
                <label class="label">
                    <span class="label-text-alt">"Allow others to see your activity timeline"</span>
                </label>
            </div>
        </div>
    }
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct PrivacyData {
    pub portfolio_visibility: PortfolioVisibility,
    pub show_activity: bool,
}
