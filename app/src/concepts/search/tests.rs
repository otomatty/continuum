#[cfg(test)]
mod tests {
    use crate::concepts::search::actions::*;
    use crate::concepts::search::state::*;

    #[test]
    fn test_update_query() {
        let state = SearchState::default();
        let new_state = update_query(state, "rust".to_string());
        assert_eq!(new_state.query, "rust");
    }

    #[test]
    fn test_clear_query() {
        let state = SearchState {
            query: "rust".to_string(),
            ..Default::default()
        };
        let new_state = clear_query(state);
        assert!(new_state.query.is_empty());
    }

    #[test]
    fn test_matches_query_case_insensitive() {
        assert!(matches_query("Hello World", "hello"));
        assert!(matches_query("Hello World", "WORLD"));
        assert!(!matches_query("Hello World", "foo"));
    }

    #[test]
    fn test_matches_empty_query() {
        assert!(matches_query("anything", ""));
    }
}

