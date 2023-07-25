use azure_work_lib::azure::PullRequestInformation;
use notify_rust::Notification;

pub struct NotificationService {
    pull_request_information: PullRequestInformation,
}

impl NotificationService {
    pub fn new() -> Self {
        Self {
            pull_request_information: PullRequestInformation::default(),
        }
    }

    pub fn notify_if_necessary(&mut self, pull_request_information: &PullRequestInformation) {
        if pull_request_information != &self.pull_request_information {
            Notification::new()
                .summary("Pull Requests To Handle")
                .body("You have pending Pull Requests to handle")
                .show()
                .ok();

            self.pull_request_information = pull_request_information.clone();
        }
    }
}
