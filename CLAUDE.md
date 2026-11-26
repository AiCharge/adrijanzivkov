# adrijanzivkov.org — Rust experiment branch

This branch rebuilds the Laravel site as a Rust web server that renders HTML. `main` still has the Laravel app.
The owner is learning Rust from this work: `LEARNING.md` has one chapter per stage, and the code carries short
teaching comments where a Rust idea first appears. Keep both up to date when you change things.

## Stack

- axum 0.8 (routing), tokio (runtime), sqlx 0.9 through its `Any` driver (SQLite locally, Postgres in
  production), askama 0.16 (templates), tower-http, axum-extra (encrypted cookie sessions), bcrypt,
  object_store (S3 uploads), rust-embed (`public/` built into the binary).
- Tailwind CSS v4 via its CLI (`assets/css/app.css` → `public/css/app.css`, `assets/css/admin.css` →
  `public/css/admin.css`) and esbuild for the admin's Tiptap editor (`assets/js/editor.js` → `public/js/editor.js`).
  The built files are committed (Laravel Cloud's Rust build has no Node); CI checks they're up to date.
- Plain scripts, no framework: `public/js/app.js` (public site), `public/js/admin.js` (admin).

## Commands

- `pnpm install && pnpm run build` — build CSS and the editor script. Commit the results in `public/`.
- `cargo run` — migrate and serve on http://localhost:3000 (run from the repository root).
- `cargo test`, `cargo fmt`, `cargo clippy --all-targets -- -D warnings` — all must be clean before committing.
- `ADMIN_PASSWORD=… cargo run -- create-admin <email> <name>` — add an admin account.

## Conventions

- Public handlers in `src/routes/<area>.rs`, admin handlers in `src/routes/admin/<area>.rs`, each with its page
  struct (`#[derive(Template)]`). Every admin handler takes `admin: Admin`, which is the login check.
- Queries live on the model structs in `src/models/`. sqlx only accepts SQL as string literals: never build SQL
  with `format!`, always `.bind()` values, use `$1, $2` placeholders, read timestamps with
  `CAST(col AS TEXT)` and write them with `CURRENT_TIMESTAMP` (the `Any` driver has no date type).
- Schema changes need a migration in both `migrations/sqlite` and `migrations/postgres`.
- Forms: a `_token` field (CSRF, `session.verify_csrf`), validation into `validation::Errors` with Macedonian
  messages, 422 with the form again on errors, flash + redirect on success.
- Templates: `templates/` (public site) and `templates/admin/` (admin, `ui-*` classes from admin.css, macros in
  `templates/admin/components/ui.html`). Use `|safe` only for HTML written in the admin.
- User-facing text is Macedonian.
