/**
 * Concepts Module
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this module):
 *   └─ app/src/lib.rs
 *
 * Dependencies (Concept modules):
 *   ├─ activity
 *   ├─ auth
 *   ├─ category
 *   ├─ contribution
 *   ├─ discussion
 *   ├─ filter
 *   ├─ github_data
 *   ├─ organization
 *   ├─ ranking
 *   ├─ repository
 *   ├─ search
 *   ├─ theme
 *   └─ user
 *
 * Related Documentation:
 *   └─ docs/00_prompts/legible-architecture.md
 */
pub mod activity;
pub mod auth;
pub mod category;
pub mod contribution;
pub mod discussion;
pub mod filter;
pub mod github_data;
pub mod organization;
pub mod ranking;
pub mod repository;
pub mod search;
pub mod theme;
pub mod user;
