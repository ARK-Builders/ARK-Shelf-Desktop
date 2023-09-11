<script lang="ts">
    import { clipboard } from '@tauri-apps/api';
    import { toast } from '@zerodevx/svelte-toast';
    import { deleteLink } from './utils';
    import { linksInfos } from '../store';
    import Scores from './Scores.svelte';

    export let url: string;
    export let name: string;
    export let score: number;
</script>

<div class="flex space-x-4 pt-2 text-sm">
    <button
        class="text-blue-400"
        on:click={() => {
            clipboard.writeText(url);
        }}>COPY</button
    >
    <button
        class="text-blue-400"
        on:click={() => {
            open(url);
        }}>OPEN</button
    >
    <button
        class="text-rose-700"
        on:click={async () => {
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
        }}>DELETE</button
    >
    <Scores {name} {score} />
</div>
