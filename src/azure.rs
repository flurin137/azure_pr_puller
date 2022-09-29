use reqwest::Client;

use crate::models::{Repository, RepositoryList, PullRequest, PullRequestList};

const VERSION: &str = "?api-version=7.1-preview.1";

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

    pub async fn get_repositories(&self) -> Result<Vec<Repository>, Box<dyn std::error::Error>> {
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
    ) -> Result<Vec<PullRequest>, Box<dyn std::error::Error>> {
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

        let pull_requests = response.json::<PullRequestList>().await?;

        Ok(pull_requests.pull_requests)
    }
}
