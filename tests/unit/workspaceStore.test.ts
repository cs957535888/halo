import { describe, it, expect, beforeEach } from 'vitest';
import { workspace } from '../../src/lib/workspace/workspaceStore.svelte';

describe('workspaceStore', () => {
  beforeEach(() => workspace.clear());

  it('starts empty', () => {
    expect(workspace.path).toBeNull();
    expect(workspace.name).toBeNull();
  });

  it('sets a path and derives the basename as name', () => {
    workspace.set('D:/work/halo');
    expect(workspace.path).toBe('D:/work/halo');
    expect(workspace.name).toBe('halo');
  });

  it('handles trailing slashes', () => {
    workspace.set('/Users/me/proj/');
    expect(workspace.name).toBe('proj');
  });

  it('handles windows-style backslashes', () => {
    workspace.set('C:\\\\Users\\\\me\\\\demo');
    expect(workspace.name).toBe('demo');
  });

  it('clears state', () => {
    workspace.set('/x');
    workspace.clear();
    expect(workspace.path).toBeNull();
  });
});
