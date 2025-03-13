<script lang="ts">
    import { getPullRequests, loadRepositories } from "./commands";
    import Header from "./Header.svelte";
    import { onMount } from "svelte";
    import type { PullRequest } from "./interfaces";
    import PullRequests from "./PullRequests.svelte";

    let pullRequests: PullRequest[] = $state([]);

    onMount(async () => {
        await loadRepositories();

        setInterval(async () => await loadPullRequests(), 60000);
        await loadPullRequests();
    });

    async function loadPullRequests(){
        pullRequests = await getPullRequests();
    }
</script>

<main class="h-full container mx-auto flex flex-col gap-5">
    <Header />

    <PullRequests {pullRequests} />
</main>
