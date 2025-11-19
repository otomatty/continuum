use chrono::{DateTime, Utc, NaiveDate};
use serde::{Serialize, Deserialize};

// User Role
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserRole {
    CurrentEmployee,
    Alumni,
    ExternalContributor,
}

// User
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub display_name: String,
    pub avatar_url: String,
    pub github_url: String,
    pub role: UserRole,
    pub joined_at: Option<DateTime<Utc>>,
    pub left_at: Option<DateTime<Utc>>,
}

// Repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub stars: u32,
    pub language: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub contributors: Vec<ContributorStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributorStats {
    pub user: User,
    pub commits: u32,
    pub percentage: f64,
}

// Activity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ActivityType {
    Commit,
    PullRequest,
    Review,
    Issue,
    Discussion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub id: String,
    pub activity_type: ActivityType,
    pub user: User,
    pub repository: Repository,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub url: String,
}

// Organization Stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationStats {
    pub total_contributors: u32,
    pub total_repositories: u32,
    pub external_prs_count: u32,
    pub total_commits: u32,
    pub period: Period,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Period {
    Weekly,
    Monthly,
    All,
}

// Ranking Entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankingEntry {
    pub user: User,
    pub commits: u32,
    pub prs: u32,
    pub reviews: u32,
    pub score: u32,
    pub rank: u32,
}

// Contribution Graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributionGraph {
    pub user: User,
    pub data: Vec<ContributionDay>,
    pub period: Period,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributionDay {
    pub date: NaiveDate,
    pub commits: u32,
    pub prs: u32,
    pub reviews: u32,
}

// Repository Contribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryContribution {
    pub repository: Repository,
    pub commits: u32,
    pub prs: u32,
    pub reviews: u32,
    pub lines_added: u32,
    pub lines_deleted: u32,
    pub percentage: f64,
}

// Mock Data Generation Functions

pub fn generate_mock_users() -> Vec<User> {
    vec![
        User {
            username: "alice-dev".to_string(),
            display_name: "Alice Developer".to_string(),
            avatar_url: "https://api.dicebear.com/7.x/avataaars/svg?seed=alice".to_string(),
            github_url: "https://github.com/alice-dev".to_string(),
            role: UserRole::CurrentEmployee,
            joined_at: Some(DateTime::parse_from_rfc3339("2022-01-15T00:00:00Z").unwrap().with_timezone(&Utc)),
            left_at: None,
        },
        User {
            username: "bob-engineer".to_string(),
            display_name: "Bob Engineer".to_string(),
            avatar_url: "https://api.dicebear.com/7.x/avataaars/svg?seed=bob".to_string(),
            github_url: "https://github.com/bob-engineer".to_string(),
            role: UserRole::CurrentEmployee,
            joined_at: Some(DateTime::parse_from_rfc3339("2021-06-01T00:00:00Z").unwrap().with_timezone(&Utc)),
            left_at: None,
        },
        User {
            username: "charlie-coder".to_string(),
            display_name: "Charlie Coder".to_string(),
            avatar_url: "https://api.dicebear.com/7.x/avataaars/svg?seed=charlie".to_string(),
            github_url: "https://github.com/charlie-coder".to_string(),
            role: UserRole::Alumni,
            joined_at: Some(DateTime::parse_from_rfc3339("2020-03-10T00:00:00Z").unwrap().with_timezone(&Utc)),
            left_at: Some(DateTime::parse_from_rfc3339("2023-12-31T00:00:00Z").unwrap().with_timezone(&Utc)),
        },
        User {
            username: "diana-hacker".to_string(),
            display_name: "Diana Hacker".to_string(),
            avatar_url: "https://api.dicebear.com/7.x/avataaars/svg?seed=diana".to_string(),
            github_url: "https://github.com/diana-hacker".to_string(),
            role: UserRole::ExternalContributor,
            joined_at: None,
            left_at: None,
        },
        User {
            username: "eve-maker".to_string(),
            display_name: "Eve Maker".to_string(),
            avatar_url: "https://api.dicebear.com/7.x/avataaars/svg?seed=eve".to_string(),
            github_url: "https://github.com/eve-maker".to_string(),
            role: UserRole::CurrentEmployee,
            joined_at: Some(DateTime::parse_from_rfc3339("2023-02-20T00:00:00Z").unwrap().with_timezone(&Utc)),
            left_at: None,
        },
    ]
}

pub fn generate_mock_repositories() -> Vec<Repository> {
    let users = generate_mock_users();
    vec![
        Repository {
            name: "awesome-rust".to_string(),
            full_name: "org/awesome-rust".to_string(),
            description: Some("A curated list of awesome Rust resources".to_string()),
            stars: 1250,
            language: Some("Rust".to_string()),
            updated_at: DateTime::parse_from_rfc3339("2024-01-15T10:30:00Z").unwrap().with_timezone(&Utc),
            contributors: vec![
                ContributorStats {
                    user: users[0].clone(),
                    commits: 45,
                    percentage: 35.5,
                },
                ContributorStats {
                    user: users[1].clone(),
                    commits: 32,
                    percentage: 25.2,
                },
            ],
        },
        Repository {
            name: "web-framework".to_string(),
            full_name: "org/web-framework".to_string(),
            description: Some("Modern web framework built with Rust".to_string()),
            stars: 890,
            language: Some("Rust".to_string()),
            updated_at: DateTime::parse_from_rfc3339("2024-01-14T15:20:00Z").unwrap().with_timezone(&Utc),
            contributors: vec![
                ContributorStats {
                    user: users[1].clone(),
                    commits: 78,
                    percentage: 42.3,
                },
                ContributorStats {
                    user: users[2].clone(),
                    commits: 28,
                    percentage: 15.2,
                },
            ],
        },
        Repository {
            name: "cli-tool".to_string(),
            full_name: "org/cli-tool".to_string(),
            description: Some("Command-line tool for developers".to_string()),
            stars: 456,
            language: Some("Rust".to_string()),
            updated_at: DateTime::parse_from_rfc3339("2024-01-13T09:15:00Z").unwrap().with_timezone(&Utc),
            contributors: vec![
                ContributorStats {
                    user: users[0].clone(),
                    commits: 23,
                    percentage: 28.7,
                },
                ContributorStats {
                    user: users[4].clone(),
                    commits: 19,
                    percentage: 23.8,
                },
            ],
        },
        Repository {
            name: "data-processor".to_string(),
            full_name: "org/data-processor".to_string(),
            description: Some("High-performance data processing library".to_string()),
            stars: 234,
            language: Some("Rust".to_string()),
            updated_at: DateTime::parse_from_rfc3339("2024-01-12T14:45:00Z").unwrap().with_timezone(&Utc),
            contributors: vec![
                ContributorStats {
                    user: users[3].clone(),
                    commits: 12,
                    percentage: 18.5,
                },
                ContributorStats {
                    user: users[1].clone(),
                    commits: 8,
                    percentage: 12.3,
                },
            ],
        },
    ]
}

pub fn generate_mock_activities() -> Vec<Activity> {
    let users = generate_mock_users();
    let repos = generate_mock_repositories();
    
    vec![
        Activity {
            id: "1".to_string(),
            activity_type: ActivityType::Commit,
            user: users[0].clone(),
            repository: repos[0].clone(),
            title: "Add new feature for async processing".to_string(),
            created_at: DateTime::parse_from_rfc3339("2024-01-15T10:30:00Z").unwrap().with_timezone(&Utc),
            url: "https://github.com/org/awesome-rust/commit/abc123".to_string(),
        },
        Activity {
            id: "2".to_string(),
            activity_type: ActivityType::PullRequest,
            user: users[1].clone(),
            repository: repos[1].clone(),
            title: "Improve error handling".to_string(),
            created_at: DateTime::parse_from_rfc3339("2024-01-14T15:20:00Z").unwrap().with_timezone(&Utc),
            url: "https://github.com/org/web-framework/pull/42".to_string(),
        },
        Activity {
            id: "3".to_string(),
            activity_type: ActivityType::Review,
            user: users[2].clone(),
            repository: repos[0].clone(),
            title: "Reviewed PR #123".to_string(),
            created_at: DateTime::parse_from_rfc3339("2024-01-14T11:15:00Z").unwrap().with_timezone(&Utc),
            url: "https://github.com/org/awesome-rust/pull/123".to_string(),
        },
        Activity {
            id: "4".to_string(),
            activity_type: ActivityType::Commit,
            user: users[3].clone(),
            repository: repos[3].clone(),
            title: "Fix memory leak in data processor".to_string(),
            created_at: DateTime::parse_from_rfc3339("2024-01-13T09:15:00Z").unwrap().with_timezone(&Utc),
            url: "https://github.com/org/data-processor/commit/def456".to_string(),
        },
        Activity {
            id: "5".to_string(),
            activity_type: ActivityType::PullRequest,
            user: users[4].clone(),
            repository: repos[2].clone(),
            title: "Add new CLI command".to_string(),
            created_at: DateTime::parse_from_rfc3339("2024-01-12T14:45:00Z").unwrap().with_timezone(&Utc),
            url: "https://github.com/org/cli-tool/pull/56".to_string(),
        },
    ]
}

pub fn generate_mock_organization_stats(period: Period) -> OrganizationStats {
    OrganizationStats {
        total_contributors: 45,
        total_repositories: 28,
        external_prs_count: 12,
        total_commits: 1234,
        period,
    }
}

pub fn generate_mock_weekly_ranking() -> Vec<RankingEntry> {
    let users = generate_mock_users();
    vec![
        RankingEntry {
            user: users[0].clone(),
            commits: 45,
            prs: 8,
            reviews: 12,
            score: 650,
            rank: 1,
        },
        RankingEntry {
            user: users[1].clone(),
            commits: 38,
            prs: 6,
            reviews: 15,
            score: 590,
            rank: 2,
        },
        RankingEntry {
            user: users[4].clone(),
            commits: 32,
            prs: 5,
            reviews: 8,
            score: 485,
            rank: 3,
        },
        RankingEntry {
            user: users[2].clone(),
            commits: 28,
            prs: 4,
            reviews: 10,
            score: 420,
            rank: 4,
        },
        RankingEntry {
            user: users[3].clone(),
            commits: 15,
            prs: 3,
            reviews: 5,
            score: 230,
            rank: 5,
        },
    ]
}

pub fn generate_mock_monthly_ranking() -> Vec<RankingEntry> {
    let users = generate_mock_users();
    vec![
        RankingEntry {
            user: users[1].clone(),
            commits: 156,
            prs: 24,
            reviews: 48,
            score: 2280,
            rank: 1,
        },
        RankingEntry {
            user: users[0].clone(),
            commits: 142,
            prs: 28,
            reviews: 42,
            score: 2120,
            rank: 2,
        },
        RankingEntry {
            user: users[4].clone(),
            commits: 98,
            prs: 18,
            reviews: 25,
            score: 1410,
            rank: 3,
        },
        RankingEntry {
            user: users[2].clone(),
            commits: 87,
            prs: 15,
            reviews: 32,
            score: 1340,
            rank: 4,
        },
        RankingEntry {
            user: users[3].clone(),
            commits: 45,
            prs: 8,
            reviews: 12,
            score: 650,
            rank: 5,
        },
    ]
}

pub fn generate_mock_contribution_graph(username: &str, period: Period) -> ContributionGraph {
    let users = generate_mock_users();
    let user = users.iter()
        .find(|u| u.username == username)
        .unwrap_or(&users[0])
        .clone();
    
    let days = match period {
        Period::Weekly => 7,
        Period::Monthly => 30,
        Period::All => 365,
    };
    
    let mut data = Vec::new();
    let base_date = NaiveDate::from_ymd_opt(2024, 1, 1).expect("Invalid date");
    
    for i in 0..days {
        let date = base_date + chrono::Duration::try_days(i as i64).unwrap_or(chrono::Duration::days(0));
        data.push(ContributionDay {
            date,
            commits: (i % 5) as u32,
            prs: (i % 3) as u32,
            reviews: (i % 4) as u32,
        });
    }
    
    ContributionGraph {
        user,
        data,
        period,
    }
}

pub fn generate_mock_repository_contributions(username: &str) -> Vec<RepositoryContribution> {
    let repos = generate_mock_repositories();
    let users = generate_mock_users();
    let _user = users.iter()
        .find(|u| u.username == username)
        .unwrap_or(&users[0]);
    
    repos.iter().map(|repo| {
        RepositoryContribution {
            repository: repo.clone(),
            commits: (repo.stars / 10) as u32,
            prs: (repo.stars / 20) as u32,
            reviews: (repo.stars / 30) as u32,
            lines_added: (repo.stars * 5) as u32,
            lines_deleted: (repo.stars * 2) as u32,
            percentage: repo.contributors.iter()
                .find(|c| c.user.username == username)
                .map(|c| c.percentage)
                .unwrap_or(0.0),
        }
    }).collect()
}

