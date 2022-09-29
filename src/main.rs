mod azure;
mod models;

use crate::azure::Azure;
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

    println!("Getting open Pull Requests for user");

    for repository in repositories {
        print!(".");
        let pull_requests = azure.get_pull_requests(&repository).await?;

        let my_prs = pull_requests
            .iter()
            .filter(|x| x.createdBy.displayName == user)
            .cloned();

        my_pull_requests.extend(my_prs);

        let prs_to_review = pull_requests
            .iter()
            .filter(|x| x.reviewers.iter().any(|r| r.displayName == user))
            .cloned();

        my_pull_requests_to_review.extend(prs_to_review);
    }

    println!();
    println!("My Pull Requests");
    for pull_request in my_pull_requests {
        println!(
            "Repository \"{}\" | PR \"{}\"",
            pull_request.repository.name, pull_request.title
        );
    }

    println!();
    println!("My Pull Requests to Review");
    for pull_request in my_pull_requests_to_review {
        println!(
            "Repository \"{}\" | PR \"{}\"",
            pull_request.repository.name, pull_request.title
        );
    }

    Ok(())
}
