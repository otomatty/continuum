/**
 * Theme Concept - Tests
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   └─ src/concepts/theme/mod.rs
 *
 * Dependencies (External files that this file imports):
 *   ├─ super::actions::*
 *   └─ super::state::*
 *
 * Related Documentation:
 *   └─ Spec: ./theme.spec.md
 */
#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use super::super::actions::*;
    use super::super::state::*;

    #[test]
    fn test_set_theme() {
        let state = ThemeState {
            current_theme: Theme::Light,
        };
        let result = set_theme(state, Theme::Dark);
        assert_eq!(result.current_theme, Theme::Dark);
    }

    #[test]
    fn test_toggle_theme_from_light() {
        let state = ThemeState {
            current_theme: Theme::Light,
        };
        let result = toggle_theme(state);
        assert_eq!(result.current_theme, Theme::Dark);
    }

    #[test]
    fn test_toggle_theme_from_dark() {
        let state = ThemeState {
            current_theme: Theme::Dark,
        };
        let result = toggle_theme(state);
        assert_eq!(result.current_theme, Theme::Light);
    }

    #[test]
    fn test_toggle_theme_from_system() {
        let state = ThemeState {
            current_theme: Theme::System,
        };
        let result = toggle_theme(state);
        assert_eq!(result.current_theme, Theme::Light);
    }

    #[test]
    fn test_get_effective_theme_light() {
        let state = ThemeState {
            current_theme: Theme::Light,
        };
        let result = get_effective_theme(&state);
        assert_eq!(result, Theme::Light);
    }

    #[test]
    fn test_get_effective_theme_dark() {
        let state = ThemeState {
            current_theme: Theme::Dark,
        };
        let result = get_effective_theme(&state);
        assert_eq!(result, Theme::Dark);
    }

    #[test]
    fn test_get_effective_theme_system() {
        let state = ThemeState {
            current_theme: Theme::System,
        };
        let result = get_effective_theme(&state);
        // System defaults to Light in the action
        assert_eq!(result, Theme::Light);
    }
}
