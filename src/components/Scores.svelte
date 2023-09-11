<script lang="ts">
    import Arrow from '~icons/mdi/arrow-up';

    export let name: string;
    export let score: number;
</script>

<button on:click={() => {}}>
    <Arrow />
</button>

<button on:click={() => {}}>
    <Arrow class="rotate-180" />
</button>

<span>Score: {Math.round(score)}</span>
