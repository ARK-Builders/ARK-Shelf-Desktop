<script lang="ts">
    import { clipboard } from '@tauri-apps/api';
    import { toast } from '@zerodevx/svelte-toast';
    import { deleteLink } from './utils';
    import { linksInfos } from '../store';
    import Score from './Score.svelte';
    import type { LinkScoreMap } from '../types';

    export let url: string;
    export let name: string;
    export let score: LinkScoreMap | undefined = undefined;
</script>

<div class="flex items-center space-x-4 pt-2 text-sm">
    <button
        class="rounded-md p-2 text-blue-400 hover:bg-blue-400 hover:bg-opacity-20"
        on:click={() => {
            clipboard.writeText(url);
        }}>COPY</button>
    <button
        class="rounded-md p-2 text-blue-400 hover:bg-blue-400 hover:bg-opacity-20"
        on:click={() => {
            open(url);
        }}>OPEN</button>
    <button
        class="rounded-md p-2 text-red-500 hover:bg-red-500 hover:bg-opacity-20"
        on:click={async () => {
            console.log('Try to delete', { name });
            const deleted = await deleteLink(name);
            if (!deleted) {
                toast.push('Error deleting link');
            } else {
                linksInfos.update(links => {
                    links = links.filter(link => link.name != name);
                    return links;
                });
                toast.push('Link deleted!');
            }
        }}>DELETE</button>
    <Score {score} {url} />
</div>
