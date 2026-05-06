<script lang="ts">
  let { onResize } = $props<{ onResize: (deltaPx: number) => void }>();

  let dragging = $state(false);
  let startX = 0;

  function down(e: PointerEvent) {
    dragging = true;
    startX = e.clientX;
    (e.target as HTMLElement).setPointerCapture(e.pointerId);
  }
  function move(e: PointerEvent) {
    if (!dragging) return;
    const delta = e.clientX - startX;
    startX = e.clientX;
    onResize(delta);
  }
  function up(e: PointerEvent) {
    if (!dragging) return;
    dragging = false;
    try {
      (e.target as HTMLElement).releasePointerCapture(e.pointerId);
    } catch {
      // capture may already be released by pointercancel — ignore
    }
  }
</script>

<div
  class="resizer"
  class:active={dragging}
  role="separator"
  aria-orientation="vertical"
  onpointerdown={down}
  onpointermove={move}
  onpointerup={up}
  onpointercancel={up}
></div>

<style>
  .resizer {
    width: 4px;
    cursor: col-resize;
    background: var(--border);
    transition: background 0.15s;
    touch-action: none;
  }
  .resizer:hover, .resizer.active { background: var(--accent); }
</style>
