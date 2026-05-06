<script lang="ts">
  import ThreePaneLayout from '$lib/layout/ThreePaneLayout.svelte';
  import WorkspaceOpener from '$lib/workspace/WorkspaceOpener.svelte';
  import FileTreeView from '$lib/filetree/FileTreeView.svelte';
  import TerminalView from '$lib/terminal/TerminalView.svelte';
  import EmptyContextPanel from '$lib/contextpanel/EmptyContextPanel.svelte';
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
    <EmptyContextPanel />
  {/snippet}
</ThreePaneLayout>
