pub const ORGANIZATION_QUERY: &str = r#"
query Organization($login: String!) {
  organization(login: $login) {
    login
    name
    description
    avatarUrl
    url
    membersWithRole {
      totalCount
    }
    repositories {
      totalCount
    }
  }
}
"#;

pub const REPOSITORIES_QUERY: &str = r#"
query Repositories($login: String!, $first: Int!, $after: String) {
  organization(login: $login) {
    repositories(first: $first, after: $after, orderBy: {field: STARGAZERS, direction: DESC}) {
      totalCount
      pageInfo {
        endCursor
        hasNextPage
      }
      nodes {
        name
        description
        url
        homepageUrl
        stargazers {
          totalCount
        }
        updatedAt
        primaryLanguage {
          name
          color
        }
        repositoryTopics(first: 5) {
          nodes {
            topic {
              name
            }
          }
        }
      }
    }
  }
}
"#;

pub const USER_QUERY: &str = r#"
query User($login: String!) {
  user(login: $login) {
    login
    name
    avatarUrl
    bio
    url
    company
    location
    createdAt
  }
}
"#;

pub const CONTRIBUTIONS_QUERY: &str = r#"
query Contributions($login: String!, $from: DateTime!, $to: DateTime!) {
  user(login: $login) {
    contributionsCollection(from: $from, to: $to) {
      totalCommitContributions
      totalPullRequestContributions
      totalPullRequestReviewContributions
      totalIssueContributions
      contributionCalendar {
        totalContributions
        weeks {
          contributionDays {
            contributionCount
            date
            color
          }
        }
      }
    }
  }
}
"#;

/// Organization の統計情報を取得するクエリ
pub const ORGANIZATION_STATS_QUERY: &str = r#"
query OrganizationStats($org: String!) {
  organization(login: $org) {
    repositories(first: 100, privacy: PUBLIC) {
      totalCount
      nodes {
        name
        stargazerCount
        forkCount
        updatedAt
        primaryLanguage {
          name
          color
        }
      }
    }
    membersWithRole {
      totalCount
    }
  }
}
"#;

/// ユーザーのコントリビューション情報を取得するクエリ（組織IDを含む）
/// 注意: `organizationID`パラメータは組織のノードID（例: "MDEyOk9yZ2FuaXphdGlvbjE="）を期待します。
/// 組織のログイン名を使用する場合は、事前に組織IDを取得する必要があります。
pub const USER_CONTRIBUTIONS_QUERY: &str = r#"
query UserContributions($username: String!, $org: ID!, $from: DateTime!, $to: DateTime!) {
  user(login: $username) {
    contributionsCollection(organizationID: $org, from: $from, to: $to) {
      totalCommitContributions
      totalPullRequestContributions
      totalPullRequestReviewContributions
      totalIssueContributions
      contributionCalendar {
        totalContributions
        weeks {
          contributionDays {
            contributionCount
            date
          }
        }
      }
    }
  }
}
"#;
