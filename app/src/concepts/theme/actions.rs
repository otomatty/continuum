/**
 * Theme Concept - Actions
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   ├─ src/concepts/theme/mod.rs
 *   └─ src/concepts/theme/tests.rs
 *
 * Dependencies (External files that this file imports):
 *   └─ ./state.rs
 *
 * Related Documentation:
 *   └─ Spec: ./theme.spec.md
 */
use super::state::{Theme, ThemeState};

/// Set the theme
pub fn set_theme(_state: ThemeState, theme: Theme) -> ThemeState {
    ThemeState {
        current_theme: theme,
    }
}

/// Toggle between Light and Dark themes
/// If current theme is System, sets it to Light
pub fn toggle_theme(state: ThemeState) -> ThemeState {
    let new_theme = match state.current_theme {
        Theme::Light => Theme::Dark,
        Theme::Dark => Theme::Light,
        Theme::System => Theme::Light,
    };
    ThemeState {
        current_theme: new_theme,
    }
}

/// Get the effective theme (Light or Dark)
/// If theme is System, returns Light as default (actual system detection would be handled in UI layer)
pub fn get_effective_theme(state: &ThemeState) -> Theme {
    match state.current_theme {
        Theme::Light => Theme::Light,
        Theme::Dark => Theme::Dark,
        Theme::System => Theme::Light, // Default to Light, actual system detection in UI
    }
}
