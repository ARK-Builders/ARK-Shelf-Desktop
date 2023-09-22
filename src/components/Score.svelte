<script lang="ts">
    import Arrow from '~icons/mdi/arrow-up';
    import type { LinkScoreMap } from '../types';
    import { addScore, createScore, substractScore } from './utils';
    import { toast } from '@zerodevx/svelte-toast';
    import { linksInfos } from '../store';

    export let score: LinkScoreMap | undefined = undefined;
    export let url: string;

    const add = async () => {
        let s: LinkScoreMap | undefined = undefined;
        if (score?.id) {
            s = await addScore(score.name);
        } else {
            s = await createScore({ value: 1, url });
        }
        if (s) {
            linksInfos.update(links => {
                const link = links.find(l => l.url === url);
                if (link) {
                    link.score = s;
                }
                return links;
            });
        } else {
            toast.push('Error updating score!');
        }
    };

    const substract = async () => {
        let s: LinkScoreMap | undefined = undefined;
        if (score?.id) {
            s = await substractScore(score.name);
        } else {
            s = await createScore({ value: -1, url });
        }
        if (s) {
            linksInfos.update(links => {
                const link = links.find(l => l.url === url);
                if (link) {
                    link.score = s;
                }
                return links;
            });
        } else {
            toast.push('Error updating score!');
        }
    };
</script>

<button on:click={add}>
    <Arrow class="h-6 w-6 text-blue-500" />
</button>

<button on:click={substract}>
    <Arrow class="h-6 w-6 rotate-180 text-red-500" />
</button>

<span>Score: {Math.round(score?.value ?? 0)}</span>
