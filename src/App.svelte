<script lang="ts">
  import { onMount } from 'svelte';
  import Form from './components/Form.svelte';
  import LinkCard from './components/LinkCard.svelte';
  import { linksInfos } from './store';
  import type { LinkInfo, SortMode } from './types';
  import { SvelteToast } from '@zerodevx/svelte-toast';
  import { readCurrentLinks } from './components/utils';
  import Loading from '~icons/line-md/loading-loop';

  let mode: SortMode = 'normal';
  let initialFetch: Promise<LinkInfo[]>;

  onMount(() => {
    initialFetch = readCurrentLinks(mode);
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
    <div class="h-full bg-neutral-950">
      <div class="flex">
        <div class="grow flex-col items-center">
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
        <Form />
      </div>
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
    --toastContainerBottom: 4rem;
    --toastContainerLeft: auto;
  }
</style>
