<script lang="ts">
    import type { PullRequest } from "./interfaces";

    let { pullRequests = $bindable() }: { pullRequests: PullRequest[] } =
        $props();
</script>

{#each pullRequests as pullRequest}
<div class="flex flex-col bg-primary p-4">
    <div>{pullRequest.status}</div>
    <a href="{pullRequest.url}" target="_blank">Click Me</a>
</div>
{/each}
