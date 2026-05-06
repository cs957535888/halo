class ExpansionStore {
  private open = $state(new Set<string>());

  isOpen(path: string): boolean {
    return this.open.has(path);
  }

  toggle(path: string) {
    const next = new Set(this.open);
    if (next.has(path)) next.delete(path);
    else next.add(path);
    this.open = next;
  }

  collapseAll() {
    this.open = new Set();
  }
}

export const expansion = new ExpansionStore();
