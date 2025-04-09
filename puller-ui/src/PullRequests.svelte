<script lang="ts">
    import CommentThread from "./CommentThreads.svelte";
    import VoteInfo from "./Reviewer.svelte";
    import type { PullRequest } from "./interfaces";

    let {
        pullRequests = $bindable(),
        title,
    }: { pullRequests: PullRequest[]; title: string } = $props();
</script>

<div class="flex flex-col gap-3">
    <div class="text-lg font-bold">{title}</div>

    {#if pullRequests.length == 0}
        <div class="p-3 rounded bg-success text-success-content font-bold">
            Nothing to do 🥳
        </div>
    {/if}
    {#each pullRequests as pullRequest}
        <a
            href={pullRequest.clean_url}
            target="_blank"
            class="flex p-3 rounded gap-4 justify-between {pullRequest.isDraft
                ? 'bg-secondary text-secondary-content'
                : 'bg-primary text-primary-content'}"
        >
            <div class="flex flex-col gap-3">
                <div class="flex gap-3">
                    <div class="font-bold">{pullRequest.pullRequestId}</div>
                    <div>{pullRequest.title}</div>
                </div>

                <div class="flex gap-x-3 gap-y-2 flex-wrap">
                    {#each pullRequest.reviewers as reviewer}
                        <VoteInfo {reviewer} />
                    {/each}
                </div>
                
                <CommentThread commentThreads={pullRequest.comment_threads} />
            </div>

            <div class="flex flex-col items-end gap-1">
                <div>{pullRequest.createdBy.displayName}</div>

                <div class="flex gap-2">
                    {#if pullRequest.isDraft}
                        <div class="badge badge-warning">draft</div>
                    {/if}
                    {#if pullRequest.mergeStatus != "succeeded"}
                        <div class="badge badge-error">Merge Issues</div>
                    {/if}
                </div>
            </div>
        </a>
    {/each}
</div>
