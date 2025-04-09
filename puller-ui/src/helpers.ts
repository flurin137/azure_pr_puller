import { Vote, type Reviewer } from "./interfaces";

interface VoteDetails {
    text: string,
    state: string,
}

export function GetVoteInfo(vote: Vote): VoteDetails {
    switch (vote) {
        case Vote.Approved:
            return { text: "Approved", state: "success" };
        case Vote.ApprovedWithSuggestions:
            return { text: "Approved with Suggestions", state: "success" };
        case Vote.NoVote:
            return { text: "No Vote", state: "info" };
        case Vote.WaitingForAuthor:
            return { text: "Waiting for Author", state: "warning" };
        case Vote.Rejected:
            return { text: "Rejected", state: "error" };
    }
}

export function GetInitials(reviewer: Reviewer): string {
    return reviewer.uniqueName.substring(0, 2).toUpperCase() + reviewer.uniqueName.substring(2, 3)
}