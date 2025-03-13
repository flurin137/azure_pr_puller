<script lang="ts">
    import { getPullRequests, loadRepositories } from "./commands";
    import Header from "./Header.svelte";
    import { onMount } from "svelte";
    import type { PullRequest } from "./interfaces";
    import PullRequests from "./PullRequests.svelte";

    let my_pull_requests_to_review: PullRequest[] = $state([]);
    let my_reviewed_pull_requests: PullRequest[] = $state([]);

    onMount(async () => {
        await loadRepositories();

        setInterval(async () => await loadPullRequests(), 60000);
        await loadPullRequests();
    });

    async function loadPullRequests(){
        let pullRequests = await getPullRequests();

        my_reviewed_pull_requests = pullRequests.my_reviewed_pull_requests;
        my_pull_requests_to_review = pullRequests.my_pull_requests_to_review;
    }
</script>

<main class="h-full container mx-auto flex flex-col gap-5">
    <Header />
    <div class="flex flex-col gap-3">
        <div class="text-lg font-bold">
            My Pull Requests to Review
        </div>
        <PullRequests pullRequests={my_pull_requests_to_review} />
        <div class="text-lg font-bold">
            My Reviewed Pull Requests
        </div>
        <PullRequests pullRequests={my_reviewed_pull_requests} />
    </div>
</main>
