<script lang="ts">
  import { toast } from '@zerodevx/svelte-toast';
  import { linksInfos } from '../store';
  import { createLink, getPreview } from './utils';
  
  export let url = '';
  export let show = false;
  let titleElement: HTMLInputElement;
  let descriptionElement: HTMLInputElement;
  let timer;
  let showMeta = false;

  const debounceAuto = async () => {
		clearTimeout(timer);
		timer = setTimeout( async () => {
      showMeta = true;
      const graph = await getPreview(url);
      if (graph) {
        titleElement.value = graph.title;
        descriptionElement.value = graph.description;
      } else {
        toast.push('Failed to fetch website data');
      }
		}, 1500);
	}
</script>

<form
  class="sticky top-0 flex flex-col space-y-2"
  on:submit|preventDefault={async e => {
    const formData = new FormData(e.currentTarget);
    const title = formData.get('title')?.toString() ?? '';
    const url = formData.get('url')?.toString() ?? '';
    const desc = formData.get('description')?.toString();
    const data = {
      title,
      url,
      desc,
    };
    const newLink = await createLink(data);
    if (newLink) {
      linksInfos.update(links => {
        links.push(newLink);
        return links;
      });
      toast.push('Link created!');
    } else {
      toast.push('Error creating link');
    }
    show = false;
  }}
>
  <label for="url" aria-label="URL" />
  <input
    type="text"
    id="url"
    name="url"
    required
    placeholder="URL*"
    class="rounded-md bg-neutral-950 px-2 py-3 outline-none ring-1 ring-neutral-500"
    bind:value={url}
    on:input={debounceAuto}
  />
  {#if showMeta}
    <label for="title" aria-label="Title" />
    <input
      type="text"
      id="title"
      name="title"
      required
      placeholder="Title*"
      bind:this={titleElement}
      class="rounded-md bg-neutral-950 px-2 py-3 outline-none ring-1 ring-neutral-500"
    />
    <label for="description" aria-label="Optional description" />
    <input
      type="text"
      name="description"
      bind:this={descriptionElement}
      placeholder="Description (Optional)"
      class="rounded-md bg-neutral-950 px-2 py-3 outline-none ring-1 ring-neutral-500"
      id="description"
    />
  {/if}
  <div class="flex justify-between">
    <button type="submit" class="pl-2 text-blue-400">CREATE</button>
  </div>
</form>
