<script lang="ts">
    import type { PullRequest } from './interfaces';

    let pullRequests : PullRequest[] = $props();
</script>

{#each pullRequests as pullRequest}
    <div></div>
    
{/each}