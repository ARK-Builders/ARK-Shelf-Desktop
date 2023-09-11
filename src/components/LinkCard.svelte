<script lang="ts">
    import type { LinkInfo, LinkScoreMap } from '../types';
    import Description from './Description.svelte';
    import LinkFooter from './LinkFooter.svelte';
    import Title from './Title.svelte';

    export let link: LinkInfo;

    let date = link.created_time ? new Date(link.created_time.secs_since_epoch * 1000) : new Date();

    const format = (value: number) => {
        return value.toString().padStart(2, '0');
    };

    const displayCreation = (secondesSinceEpoch?: number): string => {
        if (secondesSinceEpoch) {
            date.setUTCSeconds(secondesSinceEpoch);
        }
        return `${format(date.getDay())}/${format(date.getMonth())}/${date.getFullYear()} ${format(
            date.getHours(),
        )}:${format(date.getMinutes())}:${format(date.getSeconds())}`;
    };

    $: created_time = displayCreation(link.created_time?.secs_since_epoch);
</script>

<div class="w-full break-all rounded bg-neutral-850 p-4">
    <Title title={link.title} />
    <Description description={link.desc} />
    {#if link.created_time}
        <div class="text-xs">{created_time}</div>
    {/if}
    <LinkFooter name={link.name} url={link.url} score={link.score} />
</div>
