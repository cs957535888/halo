<script lang="ts">
  import ThreePaneLayout from '$lib/layout/ThreePaneLayout.svelte';
  import WorkspaceOpener from '$lib/workspace/WorkspaceOpener.svelte';
</script>

<ThreePaneLayout>
  {#snippet left()}
    <WorkspaceOpener />
    <div class="placeholder">file tree (Task 6)</div>
  {/snippet}
  {#snippet center()}
    <div class="placeholder">terminal (Task 8)</div>
  {/snippet}
  {#snippet right()}
    <div class="placeholder">context panel (Task 9)</div>
  {/snippet}
</ThreePaneLayout>

<style>
  .placeholder {
    padding: 12px;
    color: var(--fg-2);
    font-size: 12px;
  }
</style>
