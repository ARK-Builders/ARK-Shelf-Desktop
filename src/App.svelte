<script lang="ts">
    import { onMount } from 'svelte';
    import Form from './components/Form.svelte';
    import LinkCard from './components/LinkCard.svelte';
    import { linksInfos } from './store';
    import type { LinkInfo } from './types';
    import { SvelteToast } from '@zerodevx/svelte-toast';
    import { readCurrentLinks } from './components/utils';
    import Loading from '~icons/line-md/loading-loop';

    let initialFetch: Promise<LinkInfo[]>;

    let search = '';

    $: visibleLinks = $linksInfos.filter(l => {
        if (search) {
            return (
                l.url.toLocaleLowerCase().includes(search.toLocaleLowerCase()) ||
                l.title.toLocaleLowerCase().includes(search.toLocaleLowerCase()) ||
                l.desc?.toLocaleLowerCase().includes(search.toLocaleLowerCase())
            );
        } else {
            return true;
        }
    });

    onMount(() => {
        initialFetch = readCurrentLinks();
        initialFetch.then(links => {
            $linksInfos = links;
        });
    });
</script>

<div class="relative flex h-screen min-h-screen w-screen flex-col text-white">
    <h1 class="fixed flex h-14 w-full items-center bg-neutral-800 px-8 text-lg font-medium">
        ARK Shelf
    </h1>
    <main class="absolute top-14 h-[calc(100vh-3.5rem)] w-screen">
        <div class="bg-neutral-950 px-8 py-4">
            <input
                type="search"
                class="w-56 rounded-md bg-neutral-950 py-3 outline-none ring-1 ring-neutral-500"
                placeholder="Search"
                bind:value={search} />
        </div>
        <div class="flex h-full overflow-hidden overflow-y-scroll bg-neutral-950 px-8 py-4">
            <div class="flex grow flex-col items-center space-y-2 pr-4">
                {#await initialFetch}
                    <div class="flex h-full flex-col items-center justify-center">
                        <Loading class="h-12 w-12 text-white" />
                        <p>Loading links...</p>
                    </div>
                {:then}
                    {#each visibleLinks as link}
                        <LinkCard {link} />
                    {/each}
                {/await}
            </div>
            <Form />
        </div>
    </main>
</div>
<SvelteToast
    options={{
        reversed: true,
    }} />

<style>
    :root {
        --toastBarHeight: 0;
        --toastContainerTop: auto;
        --toastContainerRight: 2rem;
        --toastContainerBottom: 2rem;
        --toastContainerLeft: auto;
    }
</style>
