import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: 'tests/e2e',
  timeout: 30_000,
  use: {
    headless: true,
    baseURL: 'http://localhost:1420',
  },
  webServer: {
    command: 'pnpm dev',
    url: 'http://localhost:1420',
    timeout: 60_000,
    reuseExistingServer: !process.env.CI,
  },
});
