<script lang="ts">
  let types: any = {};
  let name: string;
  let selected: string;

  function addType() {
    if (name && selected) {
      types[name] = selected;
      name = '';
    }
  }
</script>

<h1 class="h1">Upload</h1>

<form on:submit|preventDefault={addType}>
  <label class="label mt-4">
    <span>Types</span>

    <div class="flex gap-4">
      <input class="input" type="text" placeholder="Name" bind:value={name} />
      
	  <select class="select" bind:value={selected}>
	    <option value="float">Float</option>
	    <option value="string">String</option>
	    <option value="number">Number</option>
	  </select>

      <button type="submit" class="btn variant-filled-primary">
        Add
      </button>
    </div>
  </label>
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

  <div class="flex gap-4">
    <label class="label w-full">
      <span>File</span>
      <input name="binary" class="input" type="file" required />
    </label>

    <button class="btn variant-filled-primary mt-auto" disabled={types.size === 0}>Upload</button>
  </div>
</form>
