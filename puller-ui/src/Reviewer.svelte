<script lang="ts">
    import { GetInitials, GetVoteInfo } from "./helpers";
    import type { Reviewer } from "./interfaces";

    let { reviewer }: { reviewer: Reviewer } = $props();

    let voteInfo = $derived(GetVoteInfo(reviewer.vote));
</script>

<div class="flex gap-2">
    <div class="text-sm font-bold">
        {GetInitials(reviewer)}
    </div>
    <div class="badge badge-sm badge-{voteInfo.state}">
        {voteInfo.text}
    </div>
</div>
