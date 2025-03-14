use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryList {
    pub count: i32,
    #[serde(rename = "value")]
    pub repositories: Vec<Repository>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequestList {
    pub count: i32,
    #[serde(rename = "value")]
    pub pull_requests: Vec<PullRequest>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PullRequest {
    pub repository: PullRequestRepository,
    pub pullRequestId: i32,
    pub codeReviewId: i32,
    pub status: String,
    pub createdBy: User,
    pub creationDate: String,
    pub title: String,
    pub sourceRefName: String,
    pub targetRefName: String,
    pub mergeStatus: String,
    pub isDraft: bool,
    pub mergeId: uuid::Uuid,
    pub lastMergeSourceCommit: Commit,
    pub lastMergeTargetCommit: Commit,
    pub lastMergeCommit: Option<Commit>,
    pub reviewers: Vec<Reviewer>,
    pub url: String,
    pub completionOptions: Option<CompletionOptions>,
    pub supportsIterations: bool,
    pub autoCompleteSetBy: Option<User>,
    pub statuses: Option<Vec<Status>>,
    pub clean_url: Option<String>
}

#[allow(non_snake_case, dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Statuses {
    pub value: Vec<Status>,
    pub count: i32,
}

#[allow(non_snake_case, dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Status {
    pub iterationId: i32,
    pub id: i32,
    pub state: Option<String>,
    pub description: String,
    pub creationDate: String,
    pub updatedDate: String,
    pub createdBy: User,
}

#[allow(non_snake_case, dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PullRequestRepository {
    pub id: uuid::Uuid,
    pub name: String,
    pub url: String,
    pub project: PullRequestProject,
}

#[allow(non_snake_case, dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PullRequestProject {
    pub id: uuid::Uuid,
    pub name: String,
    pub state: String,
    pub visibility: String,
    pub lastUpdateTime: String,
}

#[allow(non_snake_case, dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    pub displayName: String,
    pub url: String,
    pub _links: Links,
    pub id: uuid::Uuid,
    pub uniqueName: String,
    pub imageUrl: String,
    pub descriptor: String,
}

#[allow(non_snake_case, dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Links {
    pub avatar: Avatar,
}

#[allow(non_snake_case, dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Avatar {
    pub href: String,
}

#[allow(non_snake_case, dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Reviewer {
    pub reviewerUrl: String,
    pub vote: i32,
    pub hasDeclined: bool,
    pub isFlagged: bool,
    pub displayName: String,
    pub url: String,
    pub _links: Links,
    pub id: uuid::Uuid,
    pub uniqueName: String,
    pub imageUrl: String,
}

#[allow(non_snake_case, dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Commit {
    pub commitId: String,
    pub url: String,
}

#[allow(non_snake_case, dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CompletionOptions {
    pub mergeCommitMessage: Option<String>,
    pub deleteSourceBranch: Option<bool>,
    pub squashMerge: Option<bool>,
    pub mergeStrategy: String,
    pub transitionWorkItems: Option<bool>,
    pub autoCompleteIgnoreConfigIds: Vec<i32>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Repository {
    pub id: uuid::Uuid,
    pub name: String,
    pub url: String,
    pub project: Project,
    pub remoteUrl: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectsCollection {
    pub count: i32,
    #[serde(rename = "value")]
    pub projects: Vec<Project>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Project {
    pub id: uuid::Uuid,
    pub name: String,
    pub url: String,
    pub state: String,
}
