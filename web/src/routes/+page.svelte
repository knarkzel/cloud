<script lang="ts">
  import { onMount } from "svelte";

  interface Wasm {
    hash: string;
    title: string;
    description: string;
  }

  // Fetch recently uploaded WASM files
  let data: Wasm[];
  onMount(async () => {
    const response = await fetch("http://0.0.0.0:8000/api/wasm/read");
    data = await response.json();
  });
</script>

{#if data}
  <section class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
    {#each data as wasm}
      <a href="/run/?hash={wasm.hash}">
        <div class="shadow-lg dark:bg-surface-800 rounded">
          <img
            class="w-full"
            src="https://dummyimage.com/720x400"
            alt={wasm.title}
            width="720"
            height="400"
          />
          
          <div class="p-4">
            <p class="text-xs text-gray-500 dark:text-gray-400 truncate">{wasm.hash}</p>
            <p>{wasm.title}</p>
            <p class="my-2 text-sm leading-relaxed text-gray-600 dark:text-gray-300">
              {wasm.description}
            </p>
          </div>
        </div>
      </a>
    {/each}
  </section>
{:else}
  <h1>Loading wasm...</h1>
{/if}
