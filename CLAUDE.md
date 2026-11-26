# adrijanzivkov.org — Rust experiment branch

This branch rebuilds the Laravel site as a Rust web server that renders HTML. `main` still has the Laravel app.
The owner is learning Rust from this work: `LEARNING.md` has one chapter per stage, and the code carries short
teaching comments where a Rust idea first appears. Keep both up to date when you change things.

## Stack

- axum 0.8 (routing), tokio (runtime), sqlx 0.9 (SQLite now, read-only), askama 0.16 (templates), tower-http.
- Tailwind CSS v4 built by the Tailwind CLI: `assets/css/app.css` → `public/css/app.css`.
- One small vanilla script, `public/js/app.js` (scroll reveal, mobile menu). No JS framework.

## Commands

- `pnpm run build` (or `pnpm run dev`) — build the CSS. Run before `cargo run` on a fresh checkout.
- `cargo run` — serve on http://localhost:3000 (from the repository root: paths are relative to it).
- `cargo test`, `cargo fmt`, `cargo clippy --all-targets` — all must be clean before committing.

## Conventions

- Handlers live in `src/routes/<area>.rs`, each with its page struct (`#[derive(Template)]`).
- Queries live on the model structs in `src/models/`. sqlx 0.9 only accepts SQL as string literals: never build
  SQL with `format!`, always `.bind()` values.
- Templates: `templates/layouts` (page shells), `templates/components` (macros), `templates/partials`
  (includes), `templates/pages` (one per page). Use `|safe` only for HTML written in the admin.
- User-facing text is Macedonian.
