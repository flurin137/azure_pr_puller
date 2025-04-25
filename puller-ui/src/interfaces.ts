export interface RepositoryList {
    count: number,
    repositories: Repository[],
}

export interface PullRequestInformation {
    my_pull_requests: PullRequest[],
    my_pull_requests_to_review: PullRequest[],
    my_reviewed_pull_requests: PullRequest[],
}

export interface PullRequest {
    repository: PullRequestRepository,
    pullRequestId: number,
    codeReviewId: number,
    status: string,
    createdBy: User,
    creationDate: string,
    title: string,
    sourceRefName: string,
    targetRefName: string,
    mergeStatus: string,
    isDraft: boolean,
    mergeId: string,
    lastMergeSourceCommit: Commit,
    lastMergeTargetCommit: Commit,
    lastMergeCommit: Commit | null,
    reviewers: Reviewer[],
    url: string,
    completionOptions: CompletionOptions | null,
    supportsIterations: boolean,
    autoCompleteSetBy: User | null,
    statuses: Status[]
    comment_threads: CommentThread[],
    clean_url: string | null
}

export interface Status {
    iterationId: number,
    id: number,
    state: string | null,
    description: string,
    creationDate: string,
    updatedDate: string,
    createdBy: User,
}

export interface PullRequestRepository {
    id: string,
    name: string,
    url: string,
    project: PullRequestProject,
}

export interface PullRequestProject {
    id: string,
    name: string,
    state: string,
    visibility: string,
    lastUpdateTime: string,
}

export interface User {
    displayName: string,
    url: string,
    _links: Links,
    id: string,
    uniqueName: string,
    imageUrl: string,
    descriptor: string,
}

export interface Links {
    avatar: Avatar,
}

export interface Avatar {
    href: string,
}

export enum Vote {
    Approved = 10,
    ApprovedWithSuggestions = 5,
    NoVote = 0,
    WaitingForAuthor = -5,
    Rejected = -10,
}

export interface Reviewer {
    reviewerUrl: string,
    vote: Vote,
    hasDeclined: boolean,
    isFlagged: boolean,
    displayName: string,
    url: string,
    _links: Links,
    id: string,
    uniqueName: string,
    imageUrl: string,
}

export interface Commit {
    commitId: string,
    url: string,
}

export interface CompletionOptions {
    mergeCommitMessage: string | null,
    deleteSourceBranch: boolean | null,
    squashMerge: boolean | null,
    mergeStrategy: string,
    transitionWorkItems: boolean | null,
    autoCompleteIgnoreConfigIds: number[],
}

export interface Repository {
    id: string,
    name: string,
    url: string,
    project: Project,
    remoteUrl: string,
}

export interface ProjectsCollection {
    count: number,
    projects: Project[],
}

export interface Project {
    id: string,
    name: string,
    url: string,
    state: string,
}

export interface CommentThreads {
    value: CommentThread[],
    count: number,
}

export interface CommentThread {
    comments: Comment [],
    id: number,
    status: string | unknown,
}

export interface Comment {
    author: User,
    commentType: string,
    content: string,
    id: number,
}


export interface ConnectionConfiguration {
    username: String,
    password: String,
    url: String,
}