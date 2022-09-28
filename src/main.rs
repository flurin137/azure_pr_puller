mod models;

use dotenv::dotenv;
use models::{PullRequest, PullRequestList, Repository, RepositoryList};
use reqwest::Client;

const PASSWORD_KEY: &str = "PAT";
const USER_KEY: &str = "USER";
const URL_KEY: &str = "URL";
const VERSION: &str = "?api-version=7.1-preview.1";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv()?;
    let password = dotenv::var(PASSWORD_KEY)?;
    let user = dotenv::var(USER_KEY)?;
    let url = dotenv::var(URL_KEY)?;

    let azure = Azure::new("", &password, &url);
    let repositories = azure.get_repositories().await?;

    let mut relevant_pull_requests: Vec<PullRequest> = vec![];

    println!("Getting open Pull Requests for user");

    for repository in repositories {
        print!(".");
        let pull_requests = azure.get_pull_requests(&repository).await?;

        let pull_requests: Vec<PullRequest> = pull_requests
            .into_iter()
            .filter(|x| x.createdBy.displayName == user)
            .collect();

        for pull_request in pull_requests {
            relevant_pull_requests.push(pull_request);
        }
    }

    println!(".");

    for pull_request in relevant_pull_requests {
        println!(
            "PR \"{}\" | Repository \"{}\"",
            pull_request.title, pull_request.repository.name
        );
    }

    Ok(())
}

struct Azure {
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
