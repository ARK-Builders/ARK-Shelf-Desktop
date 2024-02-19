<script lang="ts">
    import Eye from '~icons/mdi/eye';
    import EyeOff from '~icons/mdi/eye-off';

    export let description: string | null | undefined;

    let show = true;
</script>

<div class="flex items-start pb-2">
    <p class="grow text-xs">
        {#if show && description}
            {description}
        {/if}
    </p>
    {#if description}
        <button
            class="shrink-0 pl-2"
            on:click={() => {
                show = !show;
            }}
        >
            {#if show}
                <Eye />
            {:else}
                <EyeOff />
            {/if}
        </button>
    {/if}
</div>
