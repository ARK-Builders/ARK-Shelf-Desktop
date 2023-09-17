<script lang="ts">
    import type { LinkInfo, OpenGraph } from '../types';
    import Description from './Description.svelte';
    import LinkFooter from './LinkFooter.svelte';
    import Title from './Title.svelte';
    import { Tooltip } from 'flowbite-svelte';
    import { getPreview } from './utils';

    export let link: LinkInfo;

    let date = link.created_time ? new Date(link.created_time.secs_since_epoch * 1000) : new Date();
    let previewInfo: OpenGraph | undefined = undefined;

    const format = (value: number) => {
        return value.toString().padStart(2, '0');
    };

    const updatePreview = async (url: string) => {
        const preview = await getPreview(url);
        previewInfo = preview;
    };

    const displayCreation = (secondesSinceEpoch?: number): string => {
        if (secondesSinceEpoch) {
            date.setTime(secondesSinceEpoch * 1000);
        }
        return `${format(date.getDate())}/${format(date.getMonth())}/${date.getFullYear()} ${format(
            date.getHours(),
        )}:${format(date.getMinutes())}:${format(date.getSeconds())}`;
    };

    $: created_time = displayCreation(link.created_time?.secs_since_epoch);
    $: updatePreview(link.url);
</script>

<div class="w-full break-all rounded bg-neutral-850 p-4">
    <Title title={link.title} />
    <Description description={link.desc} />
    {#if link.created_time}
        <div class="text-xs">{created_time}</div>
    {/if}
    <LinkFooter name={link.name} url={link.url} score={link.score} />
</div>
<Tooltip placement="bottom" class="z-10 mx-2 max-w-[calc(100vw-2rem)] bg-neutral-500">
    <p class="text-base">{previewInfo?.title ?? 'Preview may not be available at the moment'}</p>
    <div class="flex text-sm">
        {#if previewInfo?.image}
            <img
                src={previewInfo.image}
                alt="Preview of {link.url}"
                width="100"
                height="100"
                loading="lazy" />
        {/if}
        <p class="break-all pl-2">
            {link.desc ?? ''}
        </p>
    </div>
</Tooltip>
