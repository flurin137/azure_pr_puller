<script lang="ts">
    import type { PullRequest } from "./interfaces";

    let { pullRequests = $bindable() }: { pullRequests: PullRequest[] } =
        $props();
</script>

{#each pullRequests as pullRequest}
    <a
        href={pullRequest.url} target="_blank"
        class="flex p-4 rounded gap-4 justify-between {pullRequest.isDraft
            ? 'bg-secondary'
            : 'bg-primary'}"
    >
        <div class="flex flex-col gap-1">
            <div class="font-bold">{pullRequest.pullRequestId}</div>
            <div>{pullRequest.title}</div>
        </div>

        <div class="flex flex-col items-end gap-1">
            <div>{pullRequest.createdBy.displayName}</div>

            <div class="flex gap-2">
                {#if pullRequest.isDraft}
                    <div class="badge badge-accent">draft</div>
                {/if}
                {#if pullRequest.mergeStatus != "succeeded"}
                    <div class="badge badge-error">Merge Issues</div>
                {/if}
            </div>
        </div>
    </a>
{/each}
