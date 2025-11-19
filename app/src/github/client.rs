use reqwest::{Client, header};
use serde::Deserialize;
use serde_json::json;
use crate::github::queries::*;
use crate::github::types::*;

#[derive(Debug, thiserror::Error)]
pub enum GitHubError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("GraphQL error: {0}")]
    GraphQL(String),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("API error: {0}")]
    Api(String),
}

pub struct GitHubClient {
    client: Client,
    token: String,
    base_url: String,
}

impl GitHubClient {
    pub fn new(token: String) -> Result<Self, GitHubError> {
        Self::new_with_base_url(token, "https://api.github.com/graphql".to_string())
    }

    pub fn new_with_base_url(token: String, base_url: String) -> Result<Self, GitHubError> {
        let client = Client::builder()
            .user_agent("continuum/1.0")
            .build()?;
        
        Ok(Self { client, token, base_url })
    }

    async fn execute_query<T: for<'de> Deserialize<'de>>(&self, query: &str, variables: serde_json::Value) -> Result<T, GitHubError> {
        let res = self.client
            .post(&self.base_url)
            .header(header::AUTHORIZATION, format!("Bearer {}", self.token))
            .json(&json!({
                "query": query,
                "variables": variables
            }))
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(GitHubError::Api(format!("Status: {}", res.status())));
        }

        let mut body: serde_json::Value = res.json().await?;
        
        if let Some(errors) = body.get("errors") {
            return Err(GitHubError::GraphQL(errors.to_string()));
        }

        if let Some(data) = body.as_object_mut().and_then(|map| map.remove("data")) {
            let result: T = serde_json::from_value(data)?;
            Ok(result)
        } else {
            Err(GitHubError::GraphQL("No data found in response".to_string()))
        }
    }

    pub async fn get_organization(&self, login: &str) -> Result<Organization, GitHubError> {
        #[derive(Deserialize)]
        struct Response {
            organization: Organization,
        }

        let variables = json!({ "login": login });
        let data: Response = self.execute_query(ORGANIZATION_QUERY, variables).await?;
        Ok(data.organization)
    }

    pub async fn get_repositories(&self, login: &str, first: i64, after: Option<String>) -> Result<RepositoryConnection, GitHubError> {
        #[derive(Deserialize)]
        struct Response {
            organization: OrganizationData,
        }
        #[derive(Deserialize)]
        struct OrganizationData {
            repositories: RepositoryConnection,
        }

        let variables = json!({
            "login": login,
            "first": first,
            "after": after
        });

        let data: Response = self.execute_query(REPOSITORIES_QUERY, variables).await?;
        Ok(data.organization.repositories)
    }

    pub async fn get_user(&self, login: &str) -> Result<User, GitHubError> {
        #[derive(Deserialize)]
        struct Response {
            user: User,
        }

        let variables = json!({ "login": login });
        let data: Response = self.execute_query(USER_QUERY, variables).await?;
        Ok(data.user)
    }

    pub async fn get_contributions(&self, login: &str, from: &str, to: &str) -> Result<ContributionCollection, GitHubError> {
        #[derive(Deserialize)]
        struct Response {
            user: UserData,
        }
        #[derive(Deserialize)]
        struct UserData {
            #[serde(rename = "contributionsCollection")]
            contributions_collection: ContributionCollection,
        }

        let variables = json!({
            "login": login,
            "from": from,
            "to": to
        });

        let data: Response = self.execute_query(CONTRIBUTIONS_QUERY, variables).await?;
        Ok(data.user.contributions_collection)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path, body_json};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_organization() {
        let mock_server = MockServer::start().await;
        let client = GitHubClient::new_with_base_url("test_token".to_string(), mock_server.uri()).unwrap();

        let mock_response = json!({
            "data": {
                "organization": {
                    "login": "test_org",
                    "name": "Test Org",
                    "description": "Test Description",
                    "avatarUrl": "https://example.com/avatar.png",
                    "url": "https://github.com/test_org",
                    "membersWithRole": { "totalCount": 10 },
                    "repositories": { "totalCount": 20 }
                }
            }
        });

        Mock::given(method("POST"))
            .and(path("/"))
            .and(body_json(json!({
                "query": ORGANIZATION_QUERY,
                "variables": { "login": "test_org" }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
            .mount(&mock_server)
            .await;

        let org = client.get_organization("test_org").await.unwrap();
        assert_eq!(org.login, "test_org");
        assert_eq!(org.name, Some("Test Org".to_string()));
        assert_eq!(org.members_with_role.total_count, 10);
    }

    #[tokio::test]
    async fn test_api_error() {
        let mock_server = MockServer::start().await;
        let client = GitHubClient::new_with_base_url("test_token".to_string(), mock_server.uri()).unwrap();

        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let result = client.get_organization("test_org").await;
        assert!(matches!(result, Err(GitHubError::Api(_))));
    }

    #[tokio::test]
    async fn test_graphql_error() {
        let mock_server = MockServer::start().await;
        let client = GitHubClient::new_with_base_url("test_token".to_string(), mock_server.uri()).unwrap();

        let mock_response = json!({
            "errors": [
                { "message": "Something went wrong" }
            ]
        });

        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
            .mount(&mock_server)
            .await;

        let result = client.get_organization("test_org").await;
        assert!(matches!(result, Err(GitHubError::GraphQL(_))));
    }
}
