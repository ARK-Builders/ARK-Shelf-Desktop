<script lang="ts">
    import { toast } from '@zerodevx/svelte-toast';
    import { linksInfos } from '../store';
    import { createLink, getPreview } from './utils';
    import Calendar from '~icons/ic/baseline-calendar-month';
    import Scores from '~icons/ic/baseline-format-list-bulleted';

    const mode = linksInfos.mode;

    // Used by App.svelte
    export let url = '';
    export let show = false;

    let showMeta = false;
    let inputWaitTime = 1500
    let timer;
    let titleElement: HTMLInputElement;
    let descriptionElement: HTMLInputElement;

    // Waits inputWaitTime after every new keypress into the URL form before trying for a preview and open the meta fields
    // This might seem like a lot but most people type with staggered delays, 1500ms as a starting point seems a decent compromise
    // If delay time is hit with an invalid url,then increase inputWaitTime by 750ms to accomodate slower typing and reduce notifications. 
    const debounce = async () => {
		clearTimeout(timer);
        timer = setTimeout( async () => {
            showMeta = true;
            const graph = await getPreview(url);
            if (graph) {
                titleElement.value = graph.title ?? '';
                descriptionElement.value = graph.description ?? '';
            } else {
                inputWaitTime += 750
                toast.push('Failed to fetch website data');
            }
		}, inputWaitTime);
	}
</script>

<div class="w-56">
    <div class="flex w-full justify-between">
        <!-- Sort by Calendar Button -->
        <button
            class="rounded-md p-2"
            class:bg-green-400={$mode === 'date'}
            on:click={() => {
                linksInfos.setMode('date');
            }}
            ><Calendar />
        </button>

        <!-- Sort by Score button -->
        <button
            class="rounded-md p-2"
            class:bg-green-400={$mode === 'score'}
            on:click={() => {
                linksInfos.setMode('score');
            }}>
            <Scores />
        </button>
    </div>

    <!-- Link Creation Form -->
    <form
        class="sticky top-0 flex flex-col space-y-2"
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
            if ($linksInfos.some(link => link.url != url)) {
                toast.push("There is already a link with the same URL")
                return
            }
            if ($linksInfos.every(link => link.url !== url)) {
                const newLink = await createLink(data);
                if (newLink) {
                    linksInfos.update(links => {
                        links = links.filter(link => link.url !== url);
                        links.push(newLink);
                        return links;
                    });
                    form.reset();
                    toast.push('Link created!');
                } else {
                    toast.push('Error creating link');
                }
                show = false;
            }
        }}>

        <!-- URL Field -->
        <label for="url" aria-label="URL" />
        <input
            type="text"
            id="url"
            name="url"
            required
            placeholder="URL*"
            class="rounded-md bg-neutral-950 px-2 py-3 outline-none ring-1 ring-neutral-500"
            bind:value={url}
            on:input={debounce}
        />

        <!-- 
        Meta Fields
        If the input url debounce timeout has been triggered, show the title and description fields
        Title is autopopulated if possible  
        -->
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

        <!-- Create Button -->
        <div class="flex justify-between">
            <button type="submit" class="pl-2 text-blue-400">CREATE</button>
        </div>
    </form>
</div>
