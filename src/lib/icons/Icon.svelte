<script lang="ts">
  import * as icons from 'lucide-svelte';

  let { name, size = 16, color = 'currentColor', strokeWidth = 2 } = $props<{
    name: string;
    size?: number;
    color?: string;
    strokeWidth?: number;
  }>();

  function toPascalCase(s: string): string {
    return s
      .split('-')
      .map((p) => p.charAt(0).toUpperCase() + p.slice(1))
      .join('');
  }

  const Component = $derived((icons as Record<string, any>)[toPascalCase(name)] ?? null);
</script>

{#if Component}
  <Component {size} {color} {strokeWidth} />
{:else}
  <!-- Unknown icon name; render nothing to avoid layout shift -->
{/if}
