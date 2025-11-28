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

/// Discussion カテゴリ一覧を取得するクエリ
pub const DISCUSSION_CATEGORIES_QUERY: &str = r#"
query DiscussionCategories($owner: String!, $name: String!) {
  repository(owner: $owner, name: $name) {
    discussionCategories(first: 25) {
      nodes {
        id
        name
        description
        emoji
      }
    }
  }
}
"#;

/// Discussion 一覧を取得するクエリ
pub const DISCUSSIONS_QUERY: &str = r#"
query Discussions(
  $owner: String!, 
  $name: String!, 
  $first: Int!, 
  $after: String,
  $categoryId: ID
) {
  repository(owner: $owner, name: $name) {
    discussions(
      first: $first, 
      after: $after, 
      categoryId: $categoryId,
      orderBy: {field: CREATED_AT, direction: DESC}
    ) {
      totalCount
      pageInfo {
        hasNextPage
        hasPreviousPage
        endCursor
        startCursor
      }
      nodes {
        id
        title
        body
        bodyText
        createdAt
        updatedAt
        url
        author {
          login
          avatarUrl
        }
        category {
          id
          name
          description
          emoji
        }
        comments {
          totalCount
        }
        reactions {
          totalCount
        }
        labels(first: 10) {
          nodes {
            name
          }
        }
      }
    }
  }
}
"#;

/// 単一の Discussion を取得するクエリ
pub const DISCUSSION_DETAIL_QUERY: &str = r#"
query DiscussionDetail($id: ID!) {
  node(id: $id) {
    ... on Discussion {
      id
      title
      body
      bodyHTML
      bodyText
      createdAt
      updatedAt
      url
      author {
        login
        avatarUrl
      }
      category {
        id
        name
        description
        emoji
      }
      comments(first: 50) {
        totalCount
        nodes {
          id
          body
          bodyHTML
          createdAt
          author {
            login
            avatarUrl
          }
          reactions {
            totalCount
          }
        }
      }
      reactions {
        totalCount
      }
      labels(first: 10) {
        nodes {
          name
        }
      }
    }
  }
}
"#;
