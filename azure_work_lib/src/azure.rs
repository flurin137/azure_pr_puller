use crate::{
    azure_configuration::AzureConfiguration,
    models::{PullRequest, PullRequestList, Repository, RepositoryList, Reviewer},
};
use anyhow::anyhow;
use anyhow::Result;
use reqwest::Client;

const VERSION: &str = "?api-version=7.1-preview.1";

#[derive(Clone, Default, PartialEq)]
pub struct PullRequestInformation {
    pub my_pull_requests: Vec<PullRequest>,
    pub my_pull_requests_to_review: Vec<PullRequest>,
    pub my_reviewed_pull_requests: Vec<PullRequest>,
}

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

    pub async fn get_repositories(&self) -> Result<Vec<Repository>> {
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

    pub async fn get_pull_requests(&self, repository: &Repository) -> Result<Vec<PullRequest>> {
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

        match response.json::<PullRequestList>().await {
            Ok(pull_requests) => Ok(pull_requests.pull_requests),
            Err(err) => {
                Err(anyhow!(
                    "Unable to load Pull requests from {}. Error: {}",
                    repository.name,
                    err
                ))
            }
        }
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
    ) -> Result<PullRequestInformation> {
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
