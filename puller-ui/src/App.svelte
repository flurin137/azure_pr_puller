<script lang="ts">
    import { getPullRequests, loadRepositories } from "./commands";
    import { onMount } from "svelte";
    import type { PullRequest } from "./interfaces";
    import PullRequests from "./PullRequests.svelte";

    let my_pull_requests: PullRequest[] = $state([]);
    let my_pull_requests_to_review: PullRequest[] = $state([]);
    let my_reviewed_pull_requests: PullRequest[] = $state([]);

    let loaded = $state(false);
    let hasWorkToDo = $derived(
        my_pull_requests
            .concat(my_pull_requests_to_review)
            .concat(my_reviewed_pull_requests).length > 0,
    );

    onMount(async () => {
        await loadRepositories();
        loaded = true;
        setInterval(async () => await loadPullRequests(), 60000);
        await loadPullRequests();
    });

    async function loadPullRequests() {
        let pullRequests = await getPullRequests();

        my_pull_requests = pullRequests.my_pull_requests;
        my_reviewed_pull_requests = pullRequests.my_reviewed_pull_requests;
        my_pull_requests_to_review = pullRequests.my_pull_requests_to_review;
    }
</script>

<main class="h-full container mx-auto flex flex-col">
    {#if !loaded}
        <div class="text-5xl p-[12rem] mx-auto">...Loading...</div>
    {:else if hasWorkToDo}
        <div class="flex flex-col gap-3 p-3">
            <PullRequests
                title="My Own Pull Requests"
                pullRequests={my_pull_requests}
            />
            <PullRequests
                title="My Pull Requests to Review"
                pullRequests={my_pull_requests_to_review}
            />
            <PullRequests
                title="My Reviewed Pull Requests"
                pullRequests={my_reviewed_pull_requests}
            />
        </div>
    {:else}
        <div class="text-5xl p-[12rem] mx-auto flex flex-wrap justify-center gap-y-5">
            <div>🥳🥳</div> <div>Noting to do!!</div>  <div>🥳🥳</div>
        </div>
    {/if}
</main>
