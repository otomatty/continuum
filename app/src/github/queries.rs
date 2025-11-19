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

