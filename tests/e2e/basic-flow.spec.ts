import { test, expect } from '@playwright/test';

// These tests run against `pnpm dev` (Vite + SvelteKit SPA mode) — there is no
// Tauri runtime in this environment. Frontend code that calls `invoke(...)` will
// reject; the UI handles those rejections by surfacing error/empty states. We
// assert on those states (and on layout structure) here. Full Tauri-runtime E2E
// belongs to M4.

test('app renders three-pane layout with opener and empty states', async ({ page }) => {
  await page.goto('/');

  // Layout: three panes are present
  await expect(page.locator('aside.left')).toBeVisible();
  await expect(page.locator('main.center')).toBeVisible();
  await expect(page.locator('aside.right')).toBeVisible();

  // Workspace opener button shows default label (no Tauri runtime in this E2E)
  await expect(page.getByRole('button', { name: /Open workspace/i })).toBeVisible();

  // File tree shows empty state ("No workspace open")
  await expect(page.locator('text=No workspace open')).toBeVisible();

  // Context panel placeholder copy
  await expect(page.locator('text=Context Panel')).toBeVisible();
});

test('resizer drag changes left pane width', async ({ page }) => {
  await page.goto('/');
  const left = page.locator('aside.left');
  const before = await left.boundingBox();
  expect(before).not.toBeNull();

  // First .resizer is between left and center
  const handle = page.locator('.resizer').first();
  const hb = await handle.boundingBox();
  expect(hb).not.toBeNull();
  await page.mouse.move(hb!.x + 2, hb!.y + 10);
  await page.mouse.down();
  await page.mouse.move(hb!.x + 60, hb!.y + 10, { steps: 10 });
  await page.mouse.up();

  const after = await left.boundingBox();
  expect(after!.width).toBeGreaterThan(before!.width + 30);
});
