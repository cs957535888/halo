<script lang="ts">
  import Resizer from './Resizer.svelte';

  let { left, center, right } = $props<{
    left: import('svelte').Snippet;
    center: import('svelte').Snippet;
    right: import('svelte').Snippet;
  }>();

  let leftWidth = $state(220);
  let rightWidth = $state(360);

  function resizeLeft(d: number) {
    leftWidth = Math.max(160, Math.min(500, leftWidth + d));
  }
  function resizeRight(d: number) {
    rightWidth = Math.max(220, Math.min(700, rightWidth - d));
  }
</script>

<div class="layout" style:grid-template-columns="{leftWidth}px 4px 1fr 4px {rightWidth}px">
  <aside class="pane left">{@render left()}</aside>
  <Resizer onResize={resizeLeft} />
  <main class="pane center">{@render center()}</main>
  <Resizer onResize={resizeRight} />
  <aside class="pane right">{@render right()}</aside>
</div>

<style>
  .layout {
    display: grid;
    height: 100vh;
    grid-template-rows: 1fr;
  }
  .pane { overflow: hidden; min-width: var(--pane-min); }
  .left  { background: var(--bg-1); border-right: 1px solid var(--border); }
  .center{ background: var(--bg-0); }
  .right { background: var(--bg-1); border-left: 1px solid var(--border); }
</style>
