<script lang="ts">
  import { ProgressRadial } from "@skeletonlabs/skeleton";
  import { onMount } from "svelte";
  
  // Get hash from url then fetch WASM from database
  let preElement;
  let data: Wasm;
  let input: string;
  let output: string;
  let loading: boolean;
  $: if (data) {
    // Set defaults for json
    let json = JSON.parse(data.types);
    for (const key in json) {
      if (json.hasOwnProperty(key)) {
        const value = json[key];
        if (value === "boolean") {
          json[key] = false;
        } else if (value === "number") {
          json[key] = 0;
        } else if (value === "string") {
          json[key] = "";
        } else if (value === "array") {
          json[key] = [];
        } else if (value === "object") {
          json[key] = {};
        }
      }
    }
    input = JSON.stringify(json, null, 2);
  }
  
  onMount(async () => {
    const query = new URLSearchParams(window.location.search);
    const hash = query.get("hash");

    if (hash) {
      const response = await fetch(`http://0.0.0.0:8000/api/wasm/read/${hash}`);
      data = await response.json();
    }
  });

  // Submit function
  async function submit() {
    loading = true;
    const response = await fetch(`http://0.0.0.0:8000/api/wasm/run/${data.hash}`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: preElement.textContent
    });
    output = await response.text();
    loading = false;
  }
</script>

{#if data}
  <h1 class="h1">{data.title}</h1>

  <p class="mt-4">{data.description}</p>

  {#if input}
    <pre contenteditable="true" class="my-4" bind:this={preElement}>{input}</pre>
  {/if}
  
  <button class="btn variant-filled-primary mt-2 w-20" on:click={submit}>
    {#if loading}
	  <ProgressRadial width="w-6" stroke="200" />
    {:else}
      Run
    {/if}
  </button>

  {#if output}
    <pre class="mt-8 pre bg-surface-700">{output}</pre>
  {/if}
{:else}
  <p>Loading...</p>
{/if}
