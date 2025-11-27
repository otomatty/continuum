mod components;

use crate::components::auth_guard::AuthGuard;
use crate::components::breadcrumbs::{BreadcrumbItem, Breadcrumbs};
use crate::components::container::Container;
use crate::concepts::repository::{ContributorStats, Repository};
use crate::concepts::user::state::User;
use chrono::Utc;
use components::{ContributorPieChart, LanguageBarChart, RepositoryHeader, TopContributorsList};
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

/**
 * Repository Detail Page
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   └─ app/src/lib.rs (ルーティング)
 *
 * Dependencies (External files that this file imports):
 *   ├─ app/src/components/auth_guard/mod.rs
 *   ├─ app/src/components/breadcrumbs/mod.rs
 *   ├─ app/src/components/container/mod.rs
 *   ├─ app/src/concepts/repository/mod.rs
 *   ├─ app/src/concepts/user/state.rs
 *   └─ ./components/mod.rs
 *
 * Related Documentation:
 *   └─ docs/03_plans/continuum/tasks/task-09-repository-detail-page.md
 */

#[component]
pub fn RepositoryDetailPage() -> impl IntoView {
    view! {
        <AuthGuard>
            <RepositoryDetailContent />
        </AuthGuard>
    }
}

#[component]
fn RepositoryDetailContent() -> impl IntoView {
    let params = use_params_map();

    let owner = move || params.get().get("owner").unwrap_or_default();
    let repo_name = move || params.get().get("repo").unwrap_or_default();

    // モックデータ - 後でGitHub APIから取得するように変更
    let mock_contributors = vec![
        ContributorStats {
            user: User {
                username: "alice-dev".to_string(),
                display_name: "Alice Developer".to_string(),
                avatar_url: "https://api.dicebear.com/7.x/avataaars/svg?seed=Alice".to_string(),
                github_url: "https://github.com/alice-dev".to_string(),
                role: crate::concepts::user::state::UserRole::CurrentEmployee,
                joined_at: None,
                left_at: None,
            },
            commits: 156,
            percentage: 45.0,
        },
        ContributorStats {
            user: User {
                username: "bob-contrib".to_string(),
                display_name: "Bob Contributor".to_string(),
                avatar_url: "https://api.dicebear.com/7.x/avataaars/svg?seed=Bob".to_string(),
                github_url: "https://github.com/bob-contrib".to_string(),
                role: crate::concepts::user::state::UserRole::CurrentEmployee,
                joined_at: None,
                left_at: None,
            },
            commits: 89,
            percentage: 25.0,
        },
        ContributorStats {
            user: User {
                username: "charlie".to_string(),
                display_name: "Charlie".to_string(),
                avatar_url: "https://api.dicebear.com/7.x/avataaars/svg?seed=Charlie".to_string(),
                github_url: "https://github.com/charlie".to_string(),
                role: crate::concepts::user::state::UserRole::ExternalContributor,
                joined_at: None,
                left_at: None,
            },
            commits: 67,
            percentage: 20.0,
        },
        ContributorStats {
            user: User {
                username: "others".to_string(),
                display_name: "Others".to_string(),
                avatar_url: "https://api.dicebear.com/7.x/avataaars/svg?seed=Others".to_string(),
                github_url: "https://github.com/others".to_string(),
                role: crate::concepts::user::state::UserRole::ExternalContributor,
                joined_at: None,
                left_at: None,
            },
            commits: 34,
            percentage: 10.0,
        },
    ];

    let mock_repo = Repository {
        name: repo_name(),
        full_name: format!("{}/{}", owner(), repo_name()),
        description: Some("エンジニアの成長を支援するプラットフォーム".to_string()),
        stars: 128,
        language: Some("Rust".to_string()),
        updated_at: Utc::now(),
        contributors: mock_contributors.clone(),
    };

    let mock_languages = vec![
        ("Rust".to_string(), 75.0),
        ("TypeScript".to_string(), 15.0),
        ("CSS".to_string(), 8.0),
        ("Other".to_string(), 2.0),
    ];

    view! {
        <Container>
            <div class="space-y-8 py-8">
                // パンくずリスト
                <Breadcrumbs>
                    <BreadcrumbItem href="/repositories">
                        "リポジトリ"
                    </BreadcrumbItem>
                    <BreadcrumbItem>
                        {repo_name()}
                    </BreadcrumbItem>
                </Breadcrumbs>

                // リポジトリヘッダー
                <RepositoryHeader repository=mock_repo />

                // グラフセクション
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                    <ContributorPieChart contributors=mock_contributors.clone() />
                    <LanguageBarChart languages=mock_languages />
                </div>

                // トップコントリビューター一覧
                <TopContributorsList contributors=mock_contributors limit=10 />
            </div>
        </Container>
    }
}
