pub mod activity;
pub mod contribution;
pub mod organization;
pub mod ranking;
pub mod repository;
pub mod theme;
/**
 * Concepts Module
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this module):
 *   └─ app/src/lib.rs
 *
 * Dependencies (Concept modules):
 *   ├─ user
 *   ├─ repository
 *   ├─ activity
 *   ├─ organization
 *   ├─ ranking
 *   ├─ contribution
 *   └─ theme
 *
 * Related Documentation:
 *   └─ docs/00_prompts/legible-architecture.md
 */
pub mod user;
