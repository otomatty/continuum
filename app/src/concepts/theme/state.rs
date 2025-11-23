/**
 * Theme Concept - State
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   ├─ src/concepts/theme/mod.rs
 *   ├─ src/concepts/theme/actions.rs
 *   └─ src/concepts/theme/tests.rs
 *
 * Dependencies (External files that this file imports):
 *   └─ serde::{Deserialize, Serialize}
 *
 * Related Documentation:
 *   └─ Spec: ./theme.spec.md
 */
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    System,
}

#[derive(Debug, Clone)]
pub struct ThemeState {
    pub current_theme: Theme,
}

impl Default for ThemeState {
    fn default() -> Self {
        Self {
            current_theme: Theme::System,
        }
    }
}
