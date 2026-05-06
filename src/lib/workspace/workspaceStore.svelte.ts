function basename(p: string): string {
  const trimmed = p.replace(/[\\/]+$/, '');
  const idx = Math.max(trimmed.lastIndexOf('/'), trimmed.lastIndexOf('\\'));
  return idx >= 0 ? trimmed.slice(idx + 1) : trimmed;
}

class WorkspaceStore {
  path = $state<string | null>(null);
  name = $derived(this.path ? basename(this.path) : null);

  set(p: string) {
    this.path = p;
  }
  clear() {
    this.path = null;
  }
}

export const workspace = new WorkspaceStore();
