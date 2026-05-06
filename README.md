# Halo

A 2026 AI terminal that makes every AI-driven side effect visible.

Status: **M1 complete (v0.1 alpha)** — basic terminal foundation. Three-pane layout (file tree / xterm.js terminal / Context Panel placeholder), workspace open via Tauri dialog, cross-platform CI matrix.

Roadmap: M2 adds the Operation Observer + first three Visualizers (FileEdit / ShellExec / SshSession). M3 adds the rest of the Visualizers + Approval Prompt UI. M4 is cross-platform polish + auto-updater + beta release.

## Stack

- Tauri 2 + SvelteKit (SPA mode, `ssr = false`, `adapter-static`)
- Rust backend (PTY via `portable-pty`, file tree, IPC commands)
- xterm.js frontend (added in Task 8)
- Lucide SVG icons (no emoji)

## Dev

```bash
pnpm install
pnpm tauri dev
```

## Tests

```bash
pnpm test          # vitest unit tests
pnpm test:e2e      # Playwright (frontend only, no Tauri)
cd src-tauri && cargo test
```

## Layout

- `src/routes/+page.svelte` — root component (replaces `App.svelte` in plain Svelte setups)
- `src/routes/+layout.ts` — `ssr = false` for desktop SPA mode
- `src/lib/` — components and stores
- `src-tauri/src/` — Rust backend (pty, workspace, commands)
