/**
 * Category Concept - Tests
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   └─ src/concepts/category/mod.rs
 *
 * Dependencies (External files that this file imports):
 *   ├─ ./state.rs
 *   └─ ./actions.rs
 *
 * Related Documentation:
 *   └─ Spec: ./category.spec.md
 */

#[cfg(test)]
mod tests {
    use crate::concepts::category::actions::*;
    use crate::concepts::category::state::*;

    fn create_test_category(id: &str, name: &str, count: i32) -> Category {
        Category {
            id: id.to_string(),
            name: name.to_string(),
            description: Some(format!("Description for {}", name)),
            emoji: Some("📁".to_string()),
            discussions_count: count,
        }
    }

    // TC-001: set_categories
    #[test]
    fn test_set_categories() {
        let state = CategoryState::default();
        let categories = vec![
            create_test_category("1", "General", 10),
            create_test_category("2", "Announcements", 5),
        ];

        let new_state = set_categories(state, categories.clone());

        assert_eq!(new_state.categories.len(), 2);
        assert!(!new_state.loading);
        assert!(new_state.error.is_none());
    }

    // TC-002: select_category
    #[test]
    fn test_select_category() {
        let state = CategoryState {
            categories: vec![create_test_category("1", "General", 10)],
            ..Default::default()
        };

        let selected_state = select_category(state, Some("1".to_string()));

        assert_eq!(selected_state.selected_category_id, Some("1".to_string()));
    }

    // TC-003: select_category - deselect
    #[test]
    fn test_select_category_deselect() {
        let state = CategoryState {
            selected_category_id: Some("1".to_string()),
            ..Default::default()
        };

        let deselected_state = select_category(state, None);

        assert!(deselected_state.selected_category_id.is_none());
    }

    // TC-004: set_loading
    #[test]
    fn test_set_loading() {
        let state = CategoryState::default();

        let loading_state = set_loading(state, true);
        assert!(loading_state.loading);

        let not_loading_state = set_loading(loading_state, false);
        assert!(!not_loading_state.loading);
    }

    // TC-005: set_error
    #[test]
    fn test_set_error() {
        let state = CategoryState {
            loading: true,
            ..Default::default()
        };

        let error_state = set_error(state, "Failed to load categories".to_string());

        assert_eq!(
            error_state.error,
            Some("Failed to load categories".to_string())
        );
        assert!(!error_state.loading);
    }

    // TC-006: clear_error
    #[test]
    fn test_clear_error() {
        let state = CategoryState {
            error: Some("Previous error".to_string()),
            ..Default::default()
        };

        let cleared_state = clear_error(state);

        assert!(cleared_state.error.is_none());
    }

    // TC-007: find_category_by_id - found
    #[test]
    fn test_find_category_by_id_found() {
        let categories = vec![
            create_test_category("1", "General", 10),
            create_test_category("2", "Announcements", 5),
        ];

        let found = find_category_by_id(&categories, "1");

        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "General");
    }

    // TC-008: find_category_by_id - not found
    #[test]
    fn test_find_category_by_id_not_found() {
        let categories = vec![create_test_category("1", "General", 10)];

        let found = find_category_by_id(&categories, "999");

        assert!(found.is_none());
    }

    // TC-009: find_category_by_name - found
    #[test]
    fn test_find_category_by_name_found() {
        let categories = vec![
            create_test_category("1", "General", 10),
            create_test_category("2", "Announcements", 5),
        ];

        let found = find_category_by_name(&categories, "General");

        assert!(found.is_some());
        assert_eq!(found.unwrap().id, "1");
    }

    // TC-010: find_category_by_name - not found
    #[test]
    fn test_find_category_by_name_not_found() {
        let categories = vec![create_test_category("1", "General", 10)];

        let found = find_category_by_name(&categories, "NonExistent");

        assert!(found.is_none());
    }

    // TC-011: sort_by_discussions_count_desc
    #[test]
    fn test_sort_by_discussions_count_desc() {
        let categories = vec![
            create_test_category("1", "General", 10),
            create_test_category("2", "Announcements", 25),
            create_test_category("3", "Ideas", 15),
        ];

        let sorted = sort_by_discussions_count_desc(categories);

        assert_eq!(sorted[0].id, "2");
        assert_eq!(sorted[1].id, "3");
        assert_eq!(sorted[2].id, "1");
    }

    // TC-012: sort_by_name
    #[test]
    fn test_sort_by_name() {
        let categories = vec![
            create_test_category("1", "General", 10),
            create_test_category("2", "Announcements", 5),
            create_test_category("3", "Zulu", 15),
        ];

        let sorted = sort_by_name(categories);

        assert_eq!(sorted[0].name, "Announcements");
        assert_eq!(sorted[1].name, "General");
        assert_eq!(sorted[2].name, "Zulu");
    }

    // TC-013: Category display_name with emoji
    #[test]
    fn test_category_display_name_with_emoji() {
        let category = Category {
            id: "1".to_string(),
            name: "General".to_string(),
            description: None,
            emoji: Some("💬".to_string()),
            discussions_count: 10,
        };

        assert_eq!(category.display_name(), "💬 General");
    }

    // TC-014: Category display_name without emoji
    #[test]
    fn test_category_display_name_without_emoji() {
        let category = Category {
            id: "1".to_string(),
            name: "General".to_string(),
            description: None,
            emoji: None,
            discussions_count: 10,
        };

        assert_eq!(category.display_name(), "General");
    }

    // TC-015: get_selected_category - found
    #[test]
    fn test_get_selected_category_found() {
        let state = CategoryState {
            categories: vec![
                create_test_category("1", "General", 10),
                create_test_category("2", "Announcements", 5),
            ],
            selected_category_id: Some("1".to_string()),
            ..Default::default()
        };

        let selected = get_selected_category(&state);

        assert!(selected.is_some());
        assert_eq!(selected.unwrap().name, "General");
    }

    // TC-016: get_selected_category - not selected
    #[test]
    fn test_get_selected_category_not_selected() {
        let state = CategoryState {
            categories: vec![create_test_category("1", "General", 10)],
            selected_category_id: None,
            ..Default::default()
        };

        let selected = get_selected_category(&state);

        assert!(selected.is_none());
    }

    // TC-017: add_category
    #[test]
    fn test_add_category() {
        let state = CategoryState {
            categories: vec![create_test_category("1", "General", 10)],
            ..Default::default()
        };
        let new_category = create_test_category("2", "Ideas", 0);

        let new_state = add_category(state, new_category);

        assert_eq!(new_state.categories.len(), 2);
        assert_eq!(new_state.categories[1].name, "Ideas");
    }

    // TC-018: update_discussions_count
    #[test]
    fn test_update_discussions_count() {
        let state = CategoryState {
            categories: vec![
                create_test_category("1", "General", 10),
                create_test_category("2", "Announcements", 5),
            ],
            ..Default::default()
        };

        let new_state = update_discussions_count(state, "1", 20);

        let updated = find_category_by_id(&new_state.categories, "1").unwrap();
        assert_eq!(updated.discussions_count, 20);

        // Other category should remain unchanged
        let other = find_category_by_id(&new_state.categories, "2").unwrap();
        assert_eq!(other.discussions_count, 5);
    }
}
