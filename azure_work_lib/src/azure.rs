use std::{error::Error, fmt::Display};

use reqwest::Client;

use crate::{
    azure_configuration::AzureConfiguration,
    models::{PullRequest, PullRequestList, Repository, RepositoryList, Reviewer},
};

const VERSION: &str = "?api-version=7.1-preview.1";

#[derive(Clone, Default, PartialEq)]
pub struct PullRequestInformation {
    pub my_pull_requests: Vec<PullRequest>,
    pub my_pull_requests_to_review: Vec<PullRequest>,
    pub my_reviewed_pull_requests: Vec<PullRequest>,
}

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
    configuration: AzureConfiguration,
    client: Client,
}

impl Azure {
    pub fn new(configuration: AzureConfiguration) -> Self {
        Self {
            configuration,
            client: Client::new(),
        }
    }

    pub async fn get_repositories(&self) -> Result<Vec<Repository>, Box<dyn Error>> {
        let url = format!("{}/_apis/git/repositories{VERSION}", self.configuration.url);

        let response = self
            .client
            .get(url)
            .basic_auth(
                &self.configuration.username,
                Some(&self.configuration.password),
            )
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
            self.configuration.url
        );

        let response = self
            .client
            .get(url)
            .basic_auth(
                &self.configuration.username,
                Some(&self.configuration.password),
            )
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

    pub async fn get_clean_pull_request_url(&self, pull_request: &PullRequest) -> Option<String> {
        let project_name = pull_request.repository.project.name.clone();
        let repository_name = pull_request.repository.name.clone();
        let pull_request_id = pull_request.pullRequestId;

        let url = format!(
            "{}/{}/_git/{}/pullRequest/{}",
            &self.configuration.url, project_name, repository_name, pull_request_id
        );
        Some(url)
    }

    pub async fn get_my_pull_requests(
        &self,
        repositories: &Vec<Repository>,
    ) -> Result<PullRequestInformation, Box<dyn Error>> {
        println!("Getting open Pull Requests for user");

        let mut my_pull_requests: Vec<PullRequest> = vec![];
        let mut my_pull_requests_to_review: Vec<PullRequest> = vec![];
        let mut my_reviewed_pull_requests: Vec<PullRequest> = vec![];

        for repository in repositories {
            print!(".");
            let pull_requests = self.get_pull_requests(repository).await?;

            let my_prs = pull_requests
                .iter()
                .filter(|x| x.createdBy.displayName == self.configuration.username)
                .cloned();

            my_pull_requests.extend(my_prs);

            let is_addressed_to_me = |r: &Reviewer| r.displayName == self.configuration.username;

            let prs_to_review = pull_requests
                .iter()
                .filter(|x| {
                    x.reviewers
                        .iter()
                        .any(|r| is_addressed_to_me(r) && r.vote == 0)
                })
                .cloned();

            my_pull_requests_to_review.extend(prs_to_review);

            let reviewed_pull_requests = pull_requests
                .iter()
                .filter(|x| {
                    x.reviewers
                        .iter()
                        .any(|r| is_addressed_to_me(r) && r.vote != 0)
                })
                .cloned();

            my_reviewed_pull_requests.extend(reviewed_pull_requests);
        }

        println!();

        Ok(PullRequestInformation {
            my_pull_requests,
            my_pull_requests_to_review,
            my_reviewed_pull_requests,
        })
    }
}
