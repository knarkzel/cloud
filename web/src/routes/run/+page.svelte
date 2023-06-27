<script lang="ts">
  import { onMount } from "svelte";

  // Get hash from url then fetch WASM from database
  let data: Wasm;
  let types: Object;
  $: if (data) types = JSON.parse(data.types);
  
  onMount(async () => {
    const query = new URLSearchParams(window.location.search);
    const hash = query.get("hash");
    
    if (hash) {
      const response = await fetch(`http://0.0.0.0:8000/api/wasm/read/${hash}`);
      data = await response.json();
    }
  });

  // Utility
  function capitalize(input: string): string {
    return input.charAt(0).toUpperCase() + input.slice(1);
  }
</script>

{#if data}
  <h1 class="h1">{data.title}</h1>

  <p class="mt-4">{data.description}</p>
  
  <form method="POST" action="http://0.0.0.0:8000/api/wasm/run?hash={data.hash}">
    {#each Object.entries(types) as [name, value]}
      <label class="label my-4">
        <span>{capitalize(name)}</span>
        {#if value === "string"}
          <input name="{name}" class="input" type="text" required />
        {/if}
        {#if value === "number"}
          <input name="{name}" class="input" type="number" step="1" pattern="\d*" required>
        {/if}
        {#if value === "float"}
          <input name="{name}" class="input" type="number" step="any" required>
        {/if}
      </label>
    {/each}

    <button class="btn variant-filled-primary mt-2">Run</button>
  </form>
{:else}
  <p>Loading...</p>
{/if}
