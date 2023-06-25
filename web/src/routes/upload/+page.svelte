<h1 class="h1">Upload</h1>

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

  <div class="flex gap-2">
    <label class="label w-full">
      <span>File</span>
      <input name="binary" class="input" type="file" required />
    </label>

    <button class="btn variant-filled-primary mt-auto">Deploy!</button>
  </div>
</form>
