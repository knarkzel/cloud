<script lang='ts'>
  import '@skeletonlabs/skeleton/themes/theme-crimson.css';
  import '@skeletonlabs/skeleton/styles/skeleton.css';
  import '../app.postcss';
  
  import { AppShell, AppBar } from '@skeletonlabs/skeleton';
</script>

<AppShell>
  <svelte:fragment slot="header">
	<AppBar>
	  <svelte:fragment slot="lead">
		<a class="text-xl font-bold uppercase" href="/">Cloud</a>
	  </svelte:fragment>

      <svelte:fragment slot="trail">
        <nav>
          <a class="btn variant-filled-secondary" href="/upload">Upload</a>
        </nav>
	  </svelte:fragment>
	</AppBar>
  </svelte:fragment>

  <div class="p-4 container mx-auto">
    <slot />
  </div>
</AppShell>
