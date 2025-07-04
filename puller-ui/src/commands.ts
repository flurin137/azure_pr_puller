import { invoke } from '@tauri-apps/api/core';
import type { PullRequest, PullRequestInformation } from './interfaces';

export async function getPullRequests(): Promise<PullRequestInformation> {
    try {
        return await invoke('get_pull_requests');
    }
    catch (ex){
        console.log(ex)
        throw "";
    }
}

export async function loadRepositories(): Promise<PullRequest[]> {
    try {
        return await invoke('load_repositories');
    }
    catch (ex){
        console.log(ex)
        throw "";
    }
}
