<script lang="ts">
    import { getPullRequests, loadRepositories } from "./commands";
    import Header from "./Header.svelte";
    import { onMount } from "svelte";
    import type { PullRequest } from "./interfaces";
    import PullRequests from "./PullRequests.svelte";

    let my_pull_requests: PullRequest[] = $state([]);
    let my_pull_requests_to_review: PullRequest[] = $state([]);
    let my_reviewed_pull_requests: PullRequest[] = $state([]);

    let hasWorkToDo = $derived(my_pull_requests.concat(my_pull_requests_to_review).concat(my_reviewed_pull_requests).length > 0);

    onMount(async () => {
        await loadRepositories();

        setInterval(async () => await loadPullRequests(), 60000);
        await loadPullRequests();
    });

    async function loadPullRequests(){
        let pullRequests = await getPullRequests();

        my_pull_requests = pullRequests.my_pull_requests;
        my_reviewed_pull_requests = pullRequests.my_reviewed_pull_requests;
        my_pull_requests_to_review = pullRequests.my_pull_requests_to_review;
    }
</script>

<main class="h-full container mx-auto flex flex-col gap-5">
    <Header />
    {#if hasWorkToDo}
        <div class="flex flex-col gap-3">
            <PullRequests title="My Own Pull Requests" pullRequests={my_pull_requests} />
            <PullRequests title="My Pull Requests to Review" pullRequests={my_pull_requests_to_review} />
            <PullRequests title="My Reviewed Pull Requests" pullRequests={my_reviewed_pull_requests} />
        </div>
    {:else}
        <div class="text-5xl p-[12rem] mx-auto">
            🥳🥳 Hooray, noting to do!! 🥳🥳
        </div>
    {/if}

</main>
