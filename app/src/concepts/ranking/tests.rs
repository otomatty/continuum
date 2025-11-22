/**
 * Ranking Concept - Tests
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   └─ src/concepts/ranking/mod.rs (testモジュールとして)
 *
 * Dependencies (External files that this file imports):
 *   ├─ ./state.rs
 *   └─ ./actions.rs
 *
 * Related Documentation:
 *   └─ Spec: ./ranking.spec.md
 */

#[cfg(test)]
mod tests {
    use super::super::actions::*;
    use super::super::state::*;
    use crate::concepts::user::state::{User, UserRole};

    #[test]
    fn test_add_ranking_entry() {
        let state = RankingState::default();
        let user = User {
            username: "test-user".to_string(),
            display_name: "Test User".to_string(),
            avatar_url: "https://example.com/avatar.png".to_string(),
            github_url: "https://github.com/test-user".to_string(),
            role: UserRole::CurrentEmployee,
            joined_at: None,
            left_at: None,
        };
        let entry = RankingEntry {
            user: user.clone(),
            commits: 10,
            prs: 5,
            reviews: 3,
            score: 205,
            rank: 1,
        };

        let result = add_ranking_entry(state, entry.clone());

        assert_eq!(result.entries.len(), 1);
        assert_eq!(result.entries[0].commits, 10);
    }

    #[test]
    fn test_calculate_score() {
        let score = calculate_score(10, 5, 3);
        // 10 * 10 + 5 * 20 + 3 * 15 = 100 + 100 + 45 = 245
        assert_eq!(score, 245);
    }

    #[test]
    fn test_sort_by_score() {
        let user1 = User {
            username: "user1".to_string(),
            display_name: "User 1".to_string(),
            avatar_url: "https://example.com/avatar1.png".to_string(),
            github_url: "https://github.com/user1".to_string(),
            role: UserRole::CurrentEmployee,
            joined_at: None,
            left_at: None,
        };
        let user2 = User {
            username: "user2".to_string(),
            display_name: "User 2".to_string(),
            avatar_url: "https://example.com/avatar2.png".to_string(),
            github_url: "https://github.com/user2".to_string(),
            role: UserRole::CurrentEmployee,
            joined_at: None,
            left_at: None,
        };

        let state = RankingState {
            entries: vec![
                RankingEntry {
                    user: user1.clone(),
                    commits: 5,
                    prs: 2,
                    reviews: 1,
                    score: 100,
                    rank: 1,
                },
                RankingEntry {
                    user: user2.clone(),
                    commits: 10,
                    prs: 5,
                    reviews: 3,
                    score: 245,
                    rank: 2,
                },
            ],
        };

        let result = sort_by_score(state);

        assert_eq!(result.entries[0].score, 245);
        assert_eq!(result.entries[0].rank, 1);
        assert_eq!(result.entries[1].score, 100);
        assert_eq!(result.entries[1].rank, 2);
    }
}
