import { invoke } from '@tauri-apps/api/core';
import type { ConnectionConfiguration, PullRequest, PullRequestInformation } from './interfaces';

export async function getPullRequests(): Promise<PullRequestInformation> {
    try {
        return await invoke('get_pull_requests');
    }
    catch (ex) {
        console.log(ex)
        throw "";
    }
}

export async function loadRepositories(): Promise<PullRequest[]> {
    try {
        return await invoke('load_repositories');
    }
    catch (ex) {
        console.log(ex)
        throw "";
    }
}


export async function testConfiguration(configuration: ConnectionConfiguration): Promise<boolean> {
    try {
        return await invoke('test_configuration', { configuration: configuration });
    }
    catch (ex) {
        console.log(ex)
        return false
    }
}

export async function saveConfiguration(configuration: ConnectionConfiguration) {
    try {
        return await invoke('save_configuration', { configuration: configuration });
    }
    catch (ex) {
        console.log(ex)
        throw "";
    }
}