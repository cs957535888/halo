<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { expansion } from './fileTreeStore.svelte';
  import Icon from '$lib/icons/Icon.svelte';
  import Self from './FileTreeNode.svelte';

  let { node, depth = 0 } = $props<{
    node: { name: string; path: string; isDir: boolean };
    depth?: number;
  }>();

  let children = $state<Array<{ name: string; path: string; isDir: boolean }> | null>(null);
  let loading = $state(false);
  let error = $state<string | null>(null);

  async function load() {
    if (children !== null) return;
    loading = true;
    try {
      children = await invoke('list_dir', { path: node.path });
    } catch (e: unknown) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function onClick() {
    if (!node.isDir) return;
    expansion.toggle(node.path);
    if (expansion.isOpen(node.path)) await load();
  }

  const isOpen = $derived(node.isDir && expansion.isOpen(node.path));
</script>

<div class="row" style:padding-left="{depth * 12 + 8}px" onclick={onClick}>
  {#if node.isDir}
    <Icon name={isOpen ? 'chevron-down' : 'chevron-right'} size={12} />
    <Icon name={isOpen ? 'folder-open' : 'folder'} size={14} />
  {:else}
    <span class="spacer"></span>
    <Icon name="file" size={14} />
  {/if}
  <span class="name">{node.name}</span>
</div>

{#if isOpen}
  {#if loading}
    <div class="msg" style:padding-left="{(depth + 1) * 12 + 8}px">Loading…</div>
  {:else if error}
    <div class="msg err" style:padding-left="{(depth + 1) * 12 + 8}px">{error}</div>
  {:else if children}
    {#each children as child (child.path)}
      <Self node={child} depth={depth + 1} />
    {/each}
  {/if}
{/if}

<style>
  .row {
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 2px 8px;
    cursor: pointer;
    font-size: 12px;
    color: var(--fg-1);
    user-select: none;
  }
  .row:hover {
    background: var(--bg-2);
  }
  .spacer {
    width: 12px;
  }
  .name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .msg {
    font-size: 11px;
    color: var(--fg-2);
    padding: 2px 0;
  }
  .msg.err {
    color: var(--danger);
  }
</style>
