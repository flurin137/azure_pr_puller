<script lang="ts">
    import { saveConfiguration, testConfiguration } from "./commands";

    let url = $state("");
    let pat = $state("");
    let userName = $state("");

    let configurationValid: boolean | undefined = $state(undefined);

    function SaveConfiguration() {
        saveConfiguration({
            password: pat,
            url: url,
            username: userName,
        });
    }

    async function TestConfiguration() {
        configurationValid = await testConfiguration({
            password: pat,
            url: url,
            username: userName,
        });
    }
</script>

<form class="flex flex-col gap-5">
    <fieldset class="fieldset text-lg">
        <legend class="fieldset-legend">Server URL</legend>
        <input
            type="text"
            class="input input-lg"
            placeholder="Type here"
            bind:value={url}
        />
    </fieldset>
    <fieldset class="fieldset text-lg">
        <legend class="fieldset-legend">Username</legend>
        <input
            type="text"
            class="input input-lg"
            placeholder="Type here"
            bind:value={userName}
        />
    </fieldset>
    <fieldset class="fieldset text-lg">
        <legend class="fieldset-legend">PAT</legend>
        <input
            type="text"
            class="input input-lg"
            placeholder="Type here"
            bind:value={pat}
        />
    </fieldset>
    <div class="flex gap-5 items-center">
        <div class="btn btn-primary text-lg" onclick={SaveConfiguration}>
            Save
        </div>
        <div class="btn btn-info text-lg" onclick={TestConfiguration}>Test</div>

        {#if configurationValid !== undefined}
            {#if configurationValid}
                <div class="text-success">Configuration Valid</div>
            {:else}
                <div class="text-error">Configuration Invalid</div>
            {/if}
        {/if}
    </div>
</form>
