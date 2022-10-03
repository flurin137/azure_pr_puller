use std::{
    error::Error,
    fmt::Display,
};

use reqwest::Client;

use crate::models::{
    Project, PullRequest, PullRequestList, Repository, RepositoryList,
};

const VERSION: &str = "?api-version=7.1-preview.1";

#[derive(Debug)]
struct AzureError {
    error_message: String,
    inner_error: Box<dyn Error>,
}

impl Display for AzureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, Inner error: {}",
            self.error_message, self.inner_error
        )
    }
}

impl Error for AzureError {}

pub struct Azure {
    url: String,
    username: String,
    password: String,
    client: Client,
}

impl Azure {
    pub fn new(username: &str, password: &str, url: &str) -> Self {
        Self {
            username: username.to_owned(),
            password: password.to_owned(),
            url: url.to_owned(),
            client: Client::new(),
        }
    }

    pub async fn get_repositories(&self) -> Result<Vec<Repository>, Box<dyn Error>> {
        let url = format!("{}/_apis/git/repositories{VERSION}", self.url);

        let response = self
            .client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await?;

        let repository_list = response.json::<RepositoryList>().await?;

        Ok(repository_list.repositories)
    }

    pub async fn get_pull_requests(
        &self,
        repository: &Repository,
    ) -> Result<Vec<PullRequest>, Box<dyn Error>> {
        let repo_id = repository.id;
        let url = format!(
            "{}/_apis/git/repositories/{repo_id}/pullrequests{VERSION}",
            self.url
        );

        let response = self
            .client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await?;

        let pull_requests = match response.json::<PullRequestList>().await {
            Ok(pull_requests) => Ok(pull_requests),
            Err(err) => {
                let message = format!("Unable to load Pull requests from {}", repository.name);
                Err(AzureError {
                    error_message: message,
                    inner_error: Box::new(err),
                })
            }
        };

        Ok(pull_requests?.pull_requests)
    }

    pub async fn get_clean_pull_request_url(&self, url: &str) -> Option<String> {
        let mut parts = url.split("/");

        parts.next()?;
        parts.next()?;
        let base_url = format!("https://{}", parts.next()?);

        let project_guid = parts.next()?;
        let project = self.get_project_by_id(project_guid).await?;

        parts.next()?;
        parts.next()?;
        parts.next()?;

        let repository_guid = parts.next()?;
        let repository = self
            .get_repository_by_guid(repository_guid, &project.name)
            .await?;

        parts.next()?;
        let pull_request_id = parts.next()?;

        let url = format!(
            "{}/{}/_git/{}/pullRequest/{}",
            base_url, project.name, repository.name, pull_request_id
        );
        Some(url)
    }

    async fn get_repository_by_guid(
        &self,
        repository_guid: &str,
        project_name: &str,
    ) -> Option<Repository> {
        let repositories_url = format!(
            "{}/{project_name}/_apis/git/repositories/{repository_guid}{VERSION}",
            self.url
        );

        let response = self
            .client
            .get(repositories_url)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await
            .ok()?;
        let repository = response.json::<Repository>().await.ok()?;
        Some(repository)
    }

    async fn get_project_by_id(&self, project_guid: &str) -> Option<Project> {
        let projects_url = format!("{}/_apis/projects/{project_guid}{VERSION}", self.url);
        let response = self
            .client
            .get(projects_url)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await
            .ok()?;
        let project = response.json::<Project>().await.ok()?;
        Some(project)
    }
}
