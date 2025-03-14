<script lang="ts">
    import VoteInfo from "./Reviewer.svelte";
    import type { PullRequest } from "./interfaces";

    let {
        pullRequests = $bindable(),
        title,
    }: { pullRequests: PullRequest[]; title: string } = $props();
</script>

<div class="flex flex-col gap-3">
    <div class="text-lg font-bold">{title}</div>
    {#each pullRequests as pullRequest}
        <a
            href={pullRequest.url}
            target="_blank"
            class="flex p-4 rounded gap-4 justify-between {pullRequest.isDraft
                ? 'bg-secondary'
                : 'bg-primary'}"
        >
            <div class="flex flex-col gap-1">
                <div class="font-bold">{pullRequest.pullRequestId}</div>
                <div>{pullRequest.title}</div>

                <div class="flex gap-5">
                    {#each pullRequest.reviewers as reviewer}
                        <VoteInfo {reviewer} />
                    {/each}
                </div>
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
