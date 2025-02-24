<script lang="ts">
    import { getPullRequests } from "./commands";
    import Header from "./Header.svelte";
</script>

<main class="h-full container mx-auto flex flex-col gap-5">
    <Header />

    {#await getPullRequests()}
        <p>...rolling</p>
    {:then number}
        <p>you rolled a {number.length}!</p>
    {:catch error}
        <p style="color: red">{error.message}</p>
    {/await}
</main>
