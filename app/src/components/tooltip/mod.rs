/**
 * Tooltip Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this Component):
 *   └─ (To be added when used)
 *
 * Dependencies (External files that this Component imports):
 *   └─ leptos::prelude
 *
 * Related Documentation:
 *   ├─ Spec: ./tooltip.spec.md
 *   └─ Module: ../mod.rs
 */
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum TooltipPosition {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

#[component]
pub fn Tooltip(
    content: String,
    #[prop(optional)] position: TooltipPosition,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let position_class = match position {
        TooltipPosition::Top => "tooltip tooltip-top",
        TooltipPosition::Bottom => "tooltip tooltip-bottom",
        TooltipPosition::Left => "tooltip tooltip-left",
        TooltipPosition::Right => "tooltip tooltip-right",
    };

    let tooltip_class = if class.is_empty() {
        position_class.to_string()
    } else {
        format!("{} {}", position_class, class)
    };

    view! {
        <div class=tooltip_class data-tip=content>
            {children()}
        </div>
    }
}
