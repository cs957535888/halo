import { defineConfig } from 'vitest/config';
import { svelte } from '@sveltejs/vite-plugin-svelte';

export default defineConfig({
  plugins: [svelte({ hot: false })],
  resolve: {
    // Svelte 5 ships separate server/client builds; the client build is the one
    // that exposes `mount(...)`, which @testing-library/svelte calls under the
    // hood. Without the `browser` condition vitest picks the server build and
    // every render() throws `lifecycle_function_unavailable`.
    conditions: ['browser'],
  },
  test: {
    globals: true,
    environment: 'jsdom',
    include: ['tests/unit/**/*.test.ts'],
    setupFiles: [],
  },
});
