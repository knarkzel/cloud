<script lang="ts">
  import { onMount } from "svelte";

  // Fetch recently uploaded WASM files
  let data: Wasm[];
  onMount(async () => {
    const response = await fetch("http://0.0.0.0:8000/api/wasm/list");
    data = await response.json();
  });
</script>

{#if data}
  <section class="grid gap-8 md:grid-cols-2 lg:grid-cols-3">
    {#each data as wasm}
      <a href="/run/?hash={wasm.hash}">
        <div class="shadow-lg dark:bg-surface-800 rounded hover:scale-110 duration-200">
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

  {#if data.length === 0}
	<p>You don't have any modules. Go ahead and add one!</p>
  {/if}
{:else}
  <h1>Loading wasm...</h1>
{/if}
