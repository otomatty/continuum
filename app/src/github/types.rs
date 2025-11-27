use serde::{Deserialize, Serialize};

/// GitHub GraphQL API のレスポンス構造
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLResponse<T> {
    pub data: Option<T>,
    pub errors: Option<Vec<GraphQLError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLError {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    pub login: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub avatar_url: String,
    pub url: String,
    pub members_with_role: Count,
    pub repositories: Count,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Count {
    pub total_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RepositoryConnection {
    pub total_count: i64,
    pub page_info: PageInfo,
    pub nodes: Vec<Repository>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PageInfo {
    pub end_cursor: Option<String>,
    pub has_next_page: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Repository {
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub homepage_url: Option<String>,
    pub stargazers: Count,
    pub updated_at: String,
    pub primary_language: Option<Language>,
    pub repository_topics: RepositoryTopics,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Language {
    pub name: String,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RepositoryTopics {
    pub nodes: Vec<RepositoryTopic>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RepositoryTopic {
    pub topic: Topic,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Topic {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub login: String,
    pub name: Option<String>,
    pub avatar_url: String,
    pub bio: Option<String>,
    pub url: String,
    pub company: Option<String>,
    pub location: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContributionCollection {
    pub total_commit_contributions: i64,
    pub total_pull_request_contributions: i64,
    pub total_pull_request_review_contributions: i64,
    pub total_issue_contributions: i64,
    pub contribution_calendar: ContributionCalendar,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContributionCalendar {
    pub total_contributions: i64,
    pub weeks: Vec<ContributionCalendarWeek>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContributionCalendarWeek {
    pub contribution_days: Vec<ContributionCalendarDay>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContributionCalendarDay {
    pub contribution_count: i64,
    pub date: String,
    pub color: String,
}

/// Organization 統計のレスポンス
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrganizationStatsData {
    pub organization: Option<OrganizationStatsOrg>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationStatsOrg {
    pub repositories: OrganizationStatsRepositoryConnection,
    pub members_with_role: Count,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationStatsRepositoryConnection {
    pub total_count: i64,
    pub nodes: Vec<OrganizationStatsRepository>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationStatsRepository {
    pub name: String,
    pub stargazer_count: i64,
    pub fork_count: i64,
    pub updated_at: String,
    pub primary_language: Option<Language>,
}

/// Repositories クエリのレスポンス（既存のRepositoryConnectionを拡張）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RepositoriesData {
    pub organization: Option<RepositoriesOrg>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RepositoriesOrg {
    pub repositories: RepositoryConnection,
}
