<script lang="ts">
  let types: any = {};
  let name: string;
  let selected: string;
  let arrayType: string;
  let jsonTypes: string;

  // Update json types when types is changed
  $: jsonTypes = JSON.stringify(types);
  
  function addType() {
    if (name && selected) {
      if (arrayType && selected === "array") {
        types[name.trim()] = {
          "array": arrayType,
        };
      } else {
        types[name.trim()] = selected;
      }
      name = '';
      selected = "boolean";
      arrayType = "boolean";
    }
  }
</script>

<h1 class="h1">Upload</h1>

<form on:submit|preventDefault={addType}>
  <label class="label mt-4">
    <span>Input type layout</span>

    <div class="flex gap-4">
      <input class="input" type="text" placeholder="Name" bind:value={name} />
      
	  <select class="select" bind:value={selected}>
	    <option value="boolean">Boolean</option>
	    <option value="number">Number</option>
	    <option value="string">String</option>
	    <option value="array">Array</option>
	    <option value="object">Object</option>
	  </select>

      <button type="submit" class="btn variant-filled-primary" disabled={!name}>
        Add
      </button>
    </div>
  </label>

  {#if selected === "array"}
    <label class="label mt-4" for="">
      <span>Array of what type?</span>
      
      <select class="select mt-4" bind:value={arrayType}>
	    <option value="boolean">Boolean</option>
	    <option value="number">Number</option>
	    <option value="string">String</option>
	    <option value="object">Object</option>
	  </select>      
    </label>
  {/if}
</form>

{#if Object.keys(types).length > 0}
  <button class="btn variant-filled-secondary mt-4" on:click={() => types = {}}>
    Clear
  </button>
  
  <pre class="pre mt-4">{JSON.stringify(types, null, 2)}</pre>
{/if}

<form
  class="mt-4 space-y-4"
  method="POST"
  action="http://0.0.0.0:8000/api/wasm/create"
  enctype="multipart/form-data"
>
  <label class="label">
    <span>Title</span>
    <input name="title" class="input mt-auto" type="text" placeholder="Title" required />
  </label>

  <label class="label">
    <span>Description</span>
    <textarea
      name="description"
      class="textarea mt-4"
      placeholder="Description"
      rows="4"
      required
    />
  </label>

  <input name="types" class="input mt-auto" type="hidden" bind:value={jsonTypes} />  
  
  <div class="flex gap-4">
    <label class="label w-full">
      <span>WebAssembly file</span>
      <input name="binary" class="input" type="file" accept=".wasm" required />
    </label>

    <button class="btn variant-filled-primary mt-auto" disabled={Object.keys(types).length === 0}>Upload</button>
  </div>
</form>
