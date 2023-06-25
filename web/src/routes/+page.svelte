<script lang="ts">
  import { onMount } from "svelte";

  let data: string;
  onMount(async () => {
    const response = await fetch('http://0.0.0.0:8000/api/wasm/read');
    data = await response.text();
  });
</script>

{#if data}
  <p>{data}</p>
{:else}
  <h1>Loading wasm...</h1>
{/if}
