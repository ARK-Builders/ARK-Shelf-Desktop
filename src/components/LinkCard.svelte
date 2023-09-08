<script lang="ts">
  import type { LinkInfo } from '../types';
  import Description from './Description.svelte';
  import Title from './Title.svelte';

  export let link: LinkInfo;
</script>

<div class="bg-neutral-850 p-3">
  <Title title={link.title} />
  {#if link.desc}
    <Description description={link.desc} />
  {/if}
</div>
