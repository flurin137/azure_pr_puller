import { invoke } from '@tauri-apps/api/core';
import type { PullRequest } from './interfaces';

export async function getPullRequests(): Promise<PullRequest[]> {
    return await invoke('get_pull_requests');
}
