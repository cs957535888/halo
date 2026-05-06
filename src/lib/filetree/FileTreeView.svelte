<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { workspace } from '$lib/workspace/workspaceStore.svelte';
  import { expansion } from './fileTreeStore.svelte';
  import FileTreeNode from './FileTreeNode.svelte';

  let entries = $state<Array<{ name: string; path: string; isDir: boolean }> | null>(null);
  let error = $state<string | null>(null);

  $effect(() => {
    const path = workspace.path;
    entries = null;
    error = null;
    expansion.collapseAll();
    if (!path) return;
    invoke<typeof entries>('list_dir', { path })
      .then((r) => (entries = r))
      .catch((e) => (error = String(e)));
  });
</script>

<div class="tree">
  {#if !workspace.path}
    <div class="empty">No workspace open</div>
  {:else if error}
    <div class="empty err">{error}</div>
  {:else if entries === null}
    <div class="empty">Loading…</div>
  {:else}
    {#each entries as node (node.path)}
      <FileTreeNode {node} depth={0} />
    {/each}
  {/if}
</div>

<style>
  .tree {
    padding: 4px 0;
    overflow-y: auto;
    height: 100%;
  }
  .empty {
    padding: 12px;
    color: var(--fg-2);
    font-size: 12px;
  }
  .empty.err {
    color: var(--danger);
  }
</style>
