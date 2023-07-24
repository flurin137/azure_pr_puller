mod stdin_configuration_provider;
mod console_writer;

use crate::stdin_configuration_provider::StdInConfigurationProvider;
use azure_work_lib::{
    azure::Azure,
    azure_configuration::AzureConfiguration,
    models::{PullRequest, Repository, Reviewer},
};
use configuration::configuration_manager_factory::get_configuration_manager;
use console_writer::ConsoleWriter;
use std::error::Error;

pub struct PullRequestInformation {
    my_pull_requests: Vec<PullRequest>,
    my_pull_requests_to_review: Vec<PullRequest>,
    my_reviewed_pull_requests: Vec<PullRequest>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let fallback_configuration_provider = StdInConfigurationProvider::new_boxed();
    let configuration_manager = get_configuration_manager(fallback_configuration_provider)?;

    let config = configuration_manager.upsert_configuration()?;

    let azure = Azure::new("", &config.password, &config.url);
    let repositories = azure.get_repositories().await?;

    let pull_request_information = get_my_pull_requests(&azure, &config, &repositories).await?;

    let console_writer = ConsoleWriter::new(&azure);

    console_writer.print_pull_request_information(&pull_request_information).await;

    Ok(())
}

async fn get_my_pull_requests(
    azure: &Azure,
    config: &AzureConfiguration,
    repositories: &Vec<Repository>,
) -> Result<PullRequestInformation, Box<dyn Error>> {
    println!("Getting open Pull Requests for user");

    let mut my_pull_requests: Vec<PullRequest> = vec![];
    let mut my_pull_requests_to_review: Vec<PullRequest> = vec![];
    let mut my_reviewed_pull_requests: Vec<PullRequest> = vec![];

    for repository in repositories {
        print!(".");
        let pull_requests = azure.get_pull_requests(repository).await?;

        let my_prs = pull_requests
            .iter()
            .filter(|x| x.createdBy.displayName == config.username)
            .cloned();

        my_pull_requests.extend(my_prs);

        let is_addressed_to_me = |r: &Reviewer| r.displayName == config.username;

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
