<script lang="ts">
  import { onMount } from "svelte";
  import { CodeBlock } from '@skeletonlabs/skeleton';
  
  // Get hash from url then fetch WASM from database
  let data: Wasm;
  let types: Object;
  let output: string;
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

  // Submit function
  async function submit(event: SubmitEvent) {
    if (event.target) {
      // Send it as json
      const form = new FormData(event.target);
      const data = Object.fromEntries(form.entries());
	  const json = JSON.stringify(data);
      
      const response = await fetch(event.target.getAttribute('action'), {
        method: "POST",
        headers: {
          "Content-Type": "application/json"
        },
        body: json
      });
      output = await response.text();
    }
  }
</script>

{#if data}
  <h1 class="h1">{data.title}</h1>

  <p class="mt-4">{data.description}</p>

  <form action="http://0.0.0.0:8000/api/wasm/run/{data.hash}" on:submit|preventDefault={submit}>
    {#each Object.entries(types) as [name, value]}
      <label class="label my-4">
        <span>{capitalize(name)}</span>
        {#if value === "string"}
          <input {name} class="input" type="text" autocomplete="off" required />
        {/if}
        {#if value === "number"}
          <input {name} class="input" type="number" step="1" pattern="\d*" autocomplete="off" required />
        {/if}
        {#if value === "float"}
          <input {name} class="input" type="number" step="any" autocomplete="off" required  />
        {/if}
      </label>
    {/each}

    <button class="btn variant-filled-primary mt-2">Run</button>
  </form>

  {#if output}
    <pre class="mt-8 pre bg-surface-700">{output}</pre>
  {/if}
{:else}
  <p>Loading...</p>
{/if}
