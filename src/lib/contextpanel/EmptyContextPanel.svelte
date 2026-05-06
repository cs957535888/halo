<script lang="ts">
  import Icon from '$lib/icons/Icon.svelte';
</script>

<div class="empty">
  <div class="ico"><Icon name="eye" size={28} color="var(--fg-2)" /></div>
  <div class="title">Context Panel</div>
  <div class="sub">When an AI agent operates here, what it does will appear in this panel.</div>
  <div class="hint">(M2 enables this — file edits, SSH sessions, DB queries, etc.)</div>
</div>

<style>
  .empty {
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    padding: 24px;
    color: var(--fg-2);
  }
  .ico { margin-bottom: 12px; opacity: 0.6; }
  .title { font-size: 13px; color: var(--fg-1); margin-bottom: 4px; }
  .sub { font-size: 12px; max-width: 240px; line-height: 1.5; }
  .hint { margin-top: 12px; font-size: 11px; color: var(--fg-2); opacity: 0.7; }
</style>
