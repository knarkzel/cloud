<script lang="ts">
  const query = new URLSearchParams(window.location.search)
  const hash = query.get("hash");

</script>

<h1 class="h1">Run</h1>
{#if hash}
  <p class="mt-4">Your hash is {hash}</p>
{/if}
