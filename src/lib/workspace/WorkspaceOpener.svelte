<script lang="ts">
  import { open } from '@tauri-apps/plugin-dialog';
  import { workspace } from './workspaceStore.svelte';
  import Icon from '$lib/icons/Icon.svelte';

  async function pick() {
    const picked = await open({ directory: true, multiple: false });
    if (typeof picked === 'string') workspace.set(picked);
  }
</script>

<div class="opener">
  <button onclick={pick} class="btn">
    <Icon name="folder-open" size={14} />
    <span>{workspace.name ?? 'Open workspace'}</span>
  </button>
</div>

<style>
  .opener {
    padding: 8px;
    border-bottom: 1px solid var(--border);
  }
  .btn {
    display: flex;
    align-items: center;
    gap: 6px;
    width: 100%;
    padding: 6px 8px;
    background: var(--bg-2);
    color: var(--fg-0);
    border: 1px solid var(--border);
    border-radius: 4px;
    font-size: 12px;
    cursor: pointer;
  }
  .btn:hover {
    background: var(--bg-3);
  }
  span {
    flex: 1;
    text-align: left;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
