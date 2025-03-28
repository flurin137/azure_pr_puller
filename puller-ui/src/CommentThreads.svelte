<script lang="ts">
    import type { CommentThread } from "./interfaces";

    let {
        commentThreads = $bindable()
    }: { commentThreads: CommentThread[] } = $props();


    let fixed = $derived(commentThreads.filter(d=>d.status == "fixed").length)
    let active = $derived(commentThreads.filter(d=>d.status == "active").length)

    let color = $derived.by(() => {
        if(active == fixed && fixed > 0 )
        {
            return "success"
        }
        if(active > 0  )
        {
            return "warning"
        }

        return "info"
    })
</script>

<div class="flex items-center gap-4">
    Comments:
    <div class="badge badge-sm badge-{color}">
        {fixed} / {active + fixed}
    </div>
</div>
