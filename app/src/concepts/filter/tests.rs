#[cfg(test)]
use crate::concepts::filter::actions::*;
#[cfg(test)]
use crate::concepts::filter::state::*;

#[test]
fn test_add_language() {
    let state = FilterState::default();
    let new_state = add_language(state, "Rust".to_string());
    assert!(new_state.languages.contains("Rust"));
}

#[test]
fn test_toggle_language() {
    let state = FilterState::default();

    // 追加
    let state = toggle_language(state, "Rust".to_string());
    assert!(state.languages.contains("Rust"));

    // 削除
    let state = toggle_language(state, "Rust".to_string());
    assert!(!state.languages.contains("Rust"));
}

#[test]
fn test_has_active_filters() {
    let state = FilterState::default();
    assert!(!has_active_filters(&state));

    let state = add_language(state, "Rust".to_string());
    assert!(has_active_filters(&state));
}

#[test]
fn test_clear_all() {
    let state = add_language(FilterState::default(), "Rust".to_string());
    let state = clear_all(state);
    assert!(!has_active_filters(&state));
}
