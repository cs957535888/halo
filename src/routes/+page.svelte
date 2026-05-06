<script lang="ts">
  import ThreePaneLayout from '$lib/layout/ThreePaneLayout.svelte';
  import WorkspaceOpener from '$lib/workspace/WorkspaceOpener.svelte';
  import FileTreeView from '$lib/filetree/FileTreeView.svelte';
  import TerminalView from '$lib/terminal/TerminalView.svelte';
</script>

<ThreePaneLayout>
  {#snippet left()}
    <WorkspaceOpener />
    <FileTreeView />
  {/snippet}
  {#snippet center()}
    <TerminalView />
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
