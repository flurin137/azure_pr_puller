use azure_work_lib::{azure::{Azure, PullRequestInformation}, models::PullRequest};

pub struct ConsoleWriter<'a> {
    azure: &'a Azure<'a>,
}

impl<'a> ConsoleWriter<'a> {
    pub fn new(azure: &'a Azure) -> ConsoleWriter<'a> {
        ConsoleWriter { azure }
    }

    pub async fn print_pull_request_information(
        &self,
        pull_request_information: &PullRequestInformation,
    ) {
        println!("My Pull Requests");
        self.print_pull_requests(&pull_request_information.my_pull_requests)
            .await;

        println!();
        println!("My Pull Requests to Review");
        self.print_pull_requests(&pull_request_information.my_pull_requests_to_review)
            .await;

        println!();
        println!("My Reviewed Pull Requests");
        self.print_pull_requests(&pull_request_information.my_reviewed_pull_requests)
            .await;
    }

    async fn print_pull_requests(&self, pull_requests: &Vec<PullRequest>) {
        for pull_request in pull_requests {
            self.print_pull_request(pull_request).await;
        }
    }

    async fn print_pull_request(&self, pull_request: &PullRequest) {
        let clean_url = self
            .azure
            .get_clean_pull_request_url(pull_request)
            .await
            .unwrap_or_else(|| "".to_owned());
        println!(
            "Repository \"{}\" | PR \"{}\" | {clean_url}",
            pull_request.repository.name, pull_request.title
        );
    }
}
