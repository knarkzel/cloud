<script lang="ts">
  import "@skeletonlabs/skeleton/themes/theme-crimson.css";
  import "@skeletonlabs/skeleton/styles/skeleton.css";
  import "../app.postcss";

  import { AppShell, AppBar } from "@skeletonlabs/skeleton";
</script>

<AppShell>
  <svelte:fragment slot="header">
    <div class="bg-surface-800">
      <div class="container mx-auto">
        <AppBar>
          <svelte:fragment slot="lead">
            <a href="/">
              <img alt="Thunder" src="/thunder.png" width="64" height="64" />
            </a>
          </svelte:fragment>

          <svelte:fragment slot="trail">
            <nav>
              <a class="btn variant-filled-secondary" href="/upload">Upload</a>
            </nav>
          </svelte:fragment>
        </AppBar>
      </div>
    </div>
  </svelte:fragment>

  <div class="p-4 container mx-auto">
    <slot />
  </div>
</AppShell>
