import { render } from '@testing-library/svelte';
import { describe, it, expect } from 'vitest';
import Icon from '../../src/lib/icons/Icon.svelte';

describe('Icon', () => {
  it('renders a Lucide icon by name', () => {
    const { container } = render(Icon, { props: { name: 'folder', size: 16 } });
    const svg = container.querySelector('svg');
    expect(svg).not.toBeNull();
    expect(svg!.getAttribute('width')).toBe('16');
  });

  it('forbids passing emoji as name (compile-time hint, runtime no-op)', () => {
    const { container } = render(Icon, { props: { name: 'definitely-not-a-real-icon', size: 16 } });
    expect(container.querySelector('svg')).toBeNull();
  });
});
