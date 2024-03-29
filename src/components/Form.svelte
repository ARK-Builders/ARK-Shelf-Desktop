<script lang="ts">
    import { toast } from '@zerodevx/svelte-toast';
    import { linksInfos } from '../store';
    import { createLink, debounce, getPreview } from './utils';
    import Calendar from '~icons/ic/baseline-calendar-month';
    import Scores from '~icons/ic/baseline-format-list-bulleted';

    let url = '';
    let title = '';
    let description = '';
    const mode = linksInfos.mode;

    $: disabled = !url;

    const auto = async () => {
        if (url && title && description) {
            return;
        } else if (url) {
            const graph = await getPreview(url);
            if (graph) {
                title = graph.title ?? '';
                description = graph.description ?? '';
            } else {
                toast.push('Failed to fetch website data');
            }
        }
    };

    let error = false;

    const debouncedCheck = debounce((url: string) => {
        if ($linksInfos.some(l => l.url === url)) {
            error = true;
        } else {
            error = false;
        }
    }, 200);
</script>

<div class="sticky top-0 w-56">
    <div class="flex w-full justify-between">
        <button
            class="rounded-md p-2"
            class:bg-green-400={$mode === 'date'}
            on:click={() => {
                linksInfos.setMode('date');
            }}
            ><Calendar />
        </button>
        <button
            class="rounded-md p-2"
            class:bg-green-400={$mode === 'score'}
            on:click={() => {
                linksInfos.setMode('score');
            }}>
            <Scores />
        </button>
    </div>
    <form
        class="flex flex-col space-y-2"
        on:submit|preventDefault={async e => {
            const form = e.currentTarget;
            const formData = new FormData(form);
            const title = formData.get('title')?.toString() ?? '';
            const url = formData.get('url')?.toString() ?? '';
            const desc = formData.get('description')?.toString();
            const data = {
                title,
                url,
                desc,
            };
            if ($linksInfos.every(l => l.url !== url)) {
                const newLink = await createLink(data);
                if (newLink) {
                    linksInfos.update(links => {
                        links = links.filter(l => l.url !== url);
                        links.push(newLink);
                        return links;
                    });
                    form.reset();
                    toast.push('Link created!');
                } else {
                    toast.push('Error creating link');
                }
            }
        }}>
        <label for="url" aria-label="URL" />
        {#if error}
            <p class="break-words text-red-500">There is already a link with the same URL</p>
        {/if}
        <input
            type="text"
            id="url"
            name="url"
            required
            placeholder="URL*"
            class="rounded-md bg-neutral-950 px-2 py-3 outline-none ring-1 ring-neutral-500"
            on:keyup={e => {
                debouncedCheck(e.currentTarget.value);
            }}
            on:change={e => {
                debouncedCheck(e.currentTarget.value);
            }}
            bind:value={url}
            on:paste|preventDefault={e => {
                const text = e.clipboardData?.getData('text');
                if (text) {
                    url = text;
                    description = '';
                    title = '';
                }
            }} />
        <label for="title" aria-label="Title" />
        <input
            type="text"
            id="title"
            name="title"
            required
            placeholder="Title*"
            bind:value={title}
            class="rounded-md bg-neutral-950 px-2 py-3 outline-none ring-1 ring-neutral-500" />
        <label for="description" aria-label="Optional description" />
        <input
            type="text"
            name="description"
            bind:value={description}
            placeholder="Description (Optional)"
            class="rounded-md bg-neutral-950 px-2 py-3 outline-none ring-1 ring-neutral-500"
            id="description" />
        <div class="flex justify-between">
            <button type="submit" class="pl-2 text-blue-400" disabled={error}>CREATE</button>
            <button class="pr-2 text-rose-700" {disabled} type="button" on:click={auto}>
                AUTO FILLED
            </button>
        </div>
    </form>
</div>
