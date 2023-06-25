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
		<strong class="text-xl uppercase">Cloud</strong>
	  </svelte:fragment>

	  <svelte:fragment slot="trail">
	  </svelte:fragment>
	</AppBar>
  </svelte:fragment>

  <slot />
</AppShell>
