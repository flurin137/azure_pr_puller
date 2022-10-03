mod azure;
mod models;

use crate::{azure::Azure, models::Reviewer};
use dotenv::dotenv;
use models::PullRequest;

const PASSWORD_KEY: &str = "PAT";
const USER_KEY: &str = "USER";
const URL_KEY: &str = "URL";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv()?;
    let password = dotenv::var(PASSWORD_KEY)?;
    let user = dotenv::var(USER_KEY)?;
    let url = dotenv::var(URL_KEY)?;

    let azure = Azure::new("", &password, &url);
    let repositories = azure.get_repositories().await?;

    let mut my_pull_requests: Vec<PullRequest> = vec![];
    let mut my_pull_requests_to_review: Vec<PullRequest> = vec![];
    let mut my_reviewed_pull_requests: Vec<PullRequest> = vec![];

    println!("Getting open Pull Requests for user");

    for repository in repositories {
        print!(".");
        let pull_requests = azure.get_pull_requests(&repository).await?;

        let my_prs = pull_requests
            .iter()
            .filter(|x| x.createdBy.displayName == user)
            .cloned();

        my_pull_requests.extend(my_prs);

        let is_addressed_to_me = |r: &Reviewer| r.displayName == user;

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
    println!("My Pull Requests");
    print_pull_requests(&azure, my_pull_requests).await;

    println!();
    println!("My Pull Requests to Review");
    print_pull_requests(&azure, my_pull_requests_to_review).await;

    println!();
    println!("My Reviewed Pull Requests");
    print_pull_requests(&azure, my_reviewed_pull_requests).await;

    Ok(())
}

async fn print_pull_requests(azure: &Azure, pull_requests: Vec<PullRequest>) {
    for pull_request in pull_requests {
        print_pull_request(&azure, &pull_request).await;
    }
}

async fn print_pull_request(azure: &Azure, pull_request: &PullRequest) {
    let clean_url = azure
        .get_clean_pull_request_url(&pull_request.url)
        .await
        .unwrap_or("".to_owned());
    println!(
        "Repository \"{}\" | PR \"{}\" | {clean_url}",
        pull_request.repository.name, pull_request.title
    );
}
