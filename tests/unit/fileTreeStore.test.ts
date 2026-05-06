import { describe, it, expect, beforeEach } from 'vitest';
import { expansion } from '../../src/lib/filetree/fileTreeStore.svelte';

describe('expansion store', () => {
  beforeEach(() => expansion.collapseAll());

  it('returns false for unseen paths by default', () => {
    expect(expansion.isOpen('/a')).toBe(false);
  });

  it('toggles a path open/closed', () => {
    expansion.toggle('/a');
    expect(expansion.isOpen('/a')).toBe(true);
    expansion.toggle('/a');
    expect(expansion.isOpen('/a')).toBe(false);
  });

  it('collapseAll wipes everything', () => {
    expansion.toggle('/a');
    expansion.toggle('/b');
    expansion.collapseAll();
    expect(expansion.isOpen('/a')).toBe(false);
    expect(expansion.isOpen('/b')).toBe(false);
  });
});
