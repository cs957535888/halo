<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { Terminal } from '@xterm/xterm';
  import { FitAddon } from '@xterm/addon-fit';
  import { WebglAddon } from '@xterm/addon-webgl';
  import '@xterm/xterm/css/xterm.css';
  import { workspace } from '$lib/workspace/workspaceStore.svelte';
  import { PtyBridge } from './ptyBridge';

  let host: HTMLDivElement;
  let term: Terminal | null = null;
  let fit: FitAddon | null = null;
  let bridge: PtyBridge | null = null;
  let resizeObserver: ResizeObserver | null = null;

  onMount(async () => {
    term = new Terminal({
      fontFamily: 'ui-monospace, "JetBrains Mono", Menlo, Consolas, monospace',
      fontSize: 13,
      cursorBlink: true,
      theme: { background: '#0a0e14', foreground: '#cbd5e1' },
      scrollback: 5000,
    });
    fit = new FitAddon();
    term.loadAddon(fit);
    try {
      term.loadAddon(new WebglAddon());
    } catch {
      /* fall back to canvas */
    }
    term.open(host);
    fit.fit();

    bridge = new PtyBridge();
    await bridge.spawn({
      cols: term.cols,
      rows: term.rows,
      cwd: workspace.path,
      onData: (s) => term!.write(s),
    });

    term.onData((d) => bridge!.write(d));
    term.onResize(({ cols, rows }) => bridge!.resize(cols, rows));

    resizeObserver = new ResizeObserver(() => fit!.fit());
    resizeObserver.observe(host);
  });

  onDestroy(async () => {
    resizeObserver?.disconnect();
    await bridge?.dispose();
    term?.dispose();
  });
</script>

<div class="host" bind:this={host}></div>

<style>
  .host {
    width: 100%;
    height: 100%;
    padding: 6px;
    background: #0a0e14;
  }
</style>
