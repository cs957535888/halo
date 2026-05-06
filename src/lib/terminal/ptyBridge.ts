import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

export class PtyBridge {
  private id: string | null = null;
  private unlisten: UnlistenFn | null = null;

  async spawn(opts: {
    cols: number;
    rows: number;
    cwd?: string | null;
    onData: (s: string) => void;
  }) {
    const id = await invoke<string>('pty_spawn', {
      cols: opts.cols,
      rows: opts.rows,
      cwd: opts.cwd ?? null,
    });
    this.id = id;
    this.unlisten = await listen<string>(`pty://${id}`, (e) => opts.onData(e.payload));
  }

  async write(data: string) {
    if (!this.id) return;
    await invoke('pty_write', { id: this.id, data });
  }

  async resize(cols: number, rows: number) {
    if (!this.id) return;
    await invoke('pty_resize', { id: this.id, cols, rows });
  }

  async dispose() {
    if (this.unlisten) {
      this.unlisten();
      this.unlisten = null;
    }
    if (this.id) {
      try {
        await invoke('pty_kill', { id: this.id });
      } catch {
        /* already dead */
      }
      this.id = null;
    }
  }
}
