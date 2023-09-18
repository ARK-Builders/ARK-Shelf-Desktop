<script lang="ts">
    import { onMount } from 'svelte';
    import Form from './components/Form.svelte';
    import LinkCard from './components/LinkCard.svelte';
    import { linksInfos } from './store';
    import type { LinkInfo, SortMode } from './types';
    import { SvelteToast } from '@zerodevx/svelte-toast';
    import { readCurrentLinks } from './components/utils';
    import Loading from '~icons/line-md/loading-loop';
    import { readText } from '@tauri-apps/api/clipboard';

    let mode: SortMode = 'normal';
    let initialFetch: Promise<LinkInfo[]>;
    let showForm = false;
    let url = '';

    async function onKeyDown(event: KeyboardEvent) {
        // Open save dialog if ctrl+v is used but only if it's not already open
        // auto-fills the url field
        if (event.ctrlKey && event.key == 'v' && !showForm) {
            url = await readText() ?? "";
            showForm = !showForm;
        }
    }

    onMount(() => {
        initialFetch = readCurrentLinks(mode);
        initialFetch.then(links => {
            $linksInfos = links;
        });
    });
</script>

<div class="relative flex h-screen min-h-screen w-screen flex-col text-white">
    <h1 class="fixed flex h-14 w-full items-center justify-between bg-neutral-800 px-8 text-lg font-medium">
        ARK Shelf 
        <button on:click={() => (showForm = !showForm)} class="text-white px-4 py-2 rounded-md ml-4 border hover:bg-blue-400 border-blue-400">
            Save
        </button>
    </h1>
    <main class="absolute top-14 h-[calc(100vh-3.5rem)] w-screen">
        <div class="flex h-full overflow-hidden overflow-y-scroll bg-neutral-950 px-8 py-4">
            <div class="flex grow flex-col items-center space-y-2 pr-4">
                {#await initialFetch}
                    <div class="flex h-full flex-col items-center justify-center">
                        <Loading class="h-12 w-12 text-white" />
                        <p>Loading links...</p>
                    </div>
                {:then}
                    {#each $linksInfos as link (link.url)}
                        <LinkCard {link} />
                    {/each}
                {/await}
            </div>
            {#if showForm}
                <Form url={url} bind:show={showForm}/>
            {/if}
        </div>
    </main>
</div>
<SvelteToast
    options={{
        reversed: true,
    }}
/>

<style>
    :root {
        --toastBarHeight: 0;
        --toastContainerTop: auto;
        --toastContainerRight: 2rem;
        --toastContainerBottom: 2rem;
        --toastContainerLeft: auto;
    }
</style>

<svelte:window on:keydown={onKeyDown} />