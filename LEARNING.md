# Learning Rust by migrating this site

This branch (`experiment/rust`) rebuilds the Laravel + Inertia + Vue site as one Rust program that renders
plain HTML. `main` still has the Laravel version, so you can compare any page side by side:

```bash
git show main:resources/js/pages/News/Index.vue
```

Each chapter covers one stage: what was built, the Rust ideas it introduces, the files to read in order,
how it compares to Laravel, and a few exercises. Do the exercises — reading Rust is easy to overestimate;
the compiler teaches you when you write it.

## Running it

```bash
pnpm install          # once: Tailwind only
pnpm run build        # builds public/css/app.css from assets/css/app.css (use `pnpm run dev` to rebuild on save)
cargo run             # builds and starts the server on http://localhost:3000
```

| Laravel                     | Here                                   |
| --------------------------- | -------------------------------------- |
| `php artisan serve`         | `cargo run`                            |
| `php artisan test`          | `cargo test`                           |
| `vendor/bin/pint`           | `cargo fmt`                            |
| Larastan / PHPStan          | `cargo clippy` (and the compiler)      |
| `composer require x`        | `cargo add x`                          |
| `composer.json` / `.lock`   | `Cargo.toml` / `Cargo.lock`            |
| `routes/web.php`            | `src/routes/mod.rs`                    |
| Controllers                 | `src/routes/*.rs` (handler functions)  |
| Eloquent models             | `src/models/*.rs` (structs + SQL)      |
| Blade / Vue pages           | `templates/**/*.html` (Askama)         |
| `config/*.php` + `.env`     | `src/config.rs` + environment          |
| Exception handler           | `src/error.rs`                         |

---

## Chapter 0 — The shape of a Rust web app

**What changed.** Laravel, Inertia, Vue, Wayfinder and Ziggy are gone. What's left:

- `Cargo.toml` — the project and its *crates* (packages): **axum** (routing and HTTP, from the Tokio team),
  **tokio** (the async runtime), **sqlx** (SQL), **askama** (templates), **tower-http** (static files and request
  logs), **chrono** (dates), **anyhow** (errors), **serde** (parsing query strings).
- `src/` — the server. `templates/` — the HTML. `assets/css/` — the Tailwind source. `public/` — files served as
  they are (built CSS, the one small script, images, fonts). `storage/app/public/` — uploaded images.
- `package.json` only builds CSS now. No JavaScript framework, no bundler.

**Read, in this order**

1. `src/main.rs` — startup: read config, open the database, build the router, listen, shut down gracefully.
2. `src/config.rs` — settings from environment variables.
3. `src/state.rs` — the one thing every handler can share (the database pool).
4. `src/error.rs` — how failures become 404 and 500 responses.

**Rust ideas**

- **`Result<T, E>` and `?` instead of exceptions.** A function that can fail says so in its return type.
  `let config = Config::from_env()?;` means: if this is an error, return it now; otherwise give me the value.
  You can't forget to handle an error, because you can't get the value out without deciding what to do.
- **`Option<T>` instead of `null`.** `Option<String>` is either `Some(text)` or `None`. The compiler won't let
  you use it as a `String` until you've dealt with `None` (`match`, `if let`, `.unwrap_or_default()`, …).
- **Modules are explicit.** `mod config;` in `main.rs` is what makes `src/config.rs` part of the program.
  Nothing is autoloaded.
- **`async`/`await` needs a runtime.** `#[tokio::main]` starts Tokio. Handlers are `async fn`; `.await` waits
  without blocking the thread, so one process serves many requests at once (Laravel runs one PHP process per
  request instead).
- **Traits** are interfaces. `impl IntoResponse for AppError` (in `error.rs`) is like making a class implement
  `Responsable`. The *blanket* `impl<E> From<E> for AppError` makes `?` turn any sqlx/Askama/IO error into a 500.

**Compared to Laravel.** Laravel's exception handler catches whatever is thrown. Here, every place that can
fail is visible in the code as a `?`, and the compiler checks that all of them lead to `AppError`.

**Exercises**

1. Start the server with `PORT=abc cargo run`. Read the error, then find the line in `config.rs` that produced it.
2. In `main.rs`, delete one `?`. Read the compiler's message — it explains exactly what you skipped.
3. Add a `/health` route next to `/up` in `src/routes/mod.rs` that returns `"healthy"`.

---

## Chapter 1 — The public site, read-only

**What changed.** Every public page now comes from Rust: home, about, news (list + post), donation needs
(list + page), terms, privacy, the story form (display only), and a 404 page. The database is opened
**read-only**, so this stage can't change any data. The story form *sends* in Chapter 2.

**Read, in this order**

1. `src/routes/mod.rs` — the route list and static files.
2. `src/routes/news.rs` — two handlers: `index` (paginated list) and `show` (one post by slug).
3. `src/models/news.rs` — the `News` struct and its three queries.
4. `src/pagination.rs` — `Paginator<T>` and Laravel's page-link window, with tests at the bottom.
5. `templates/pages/news/index.html` → `templates/layouts/default.html` → `templates/layouts/base.html`.
6. `templates/components/cards.html` and `sections.html` — the macros that replaced Vue components.
7. `src/site.rs` — the custom `Site` extractor and the `Layout` (SEO tags) every page gets.
8. `src/routes/home.rs` — two queries run at the same time with `tokio::try_join!`.
9. `public/js/app.js` — the ~80 lines of JavaScript that replaced Vue on the public site.

**Rust ideas**

- **Extractors.** A handler's parameters are filled from the request by their *type*:
  `State(state): State<AppState>`, `Path(slug): Path<String>`, `Query(query): Query<PageQuery>`, and our own
  `site: Site` (see `site.rs`, which implements `FromRequestParts`). This is axum's version of Laravel's
  method injection, but each extractor takes exactly one thing, and a missing one is a compile error.
- **Structs + derives instead of magic properties.** `#[derive(sqlx::FromRow)]` writes the code that fills a
  `News` from a row; `#[derive(Template)]` compiles a template into Rust code tied to a struct. A template that
  uses a field the struct doesn't have **fails to compile** — no "undefined variable" at runtime.
- **Borrowing.** `fn image_url(&self)` *borrows* the record: it can read it but not keep or change it.
  `&state.db` lends the pool to a query without giving it away. Most "fighting the borrow checker" is about
  this: who owns a value, and who's only looking at it.
- **Generics.** `Paginator<T>` works for `News` and `DonationNeed`; the compiler checks each use separately.
- **`'static` text.** The verses, pillars and values (`home.rs`, `about.rs`) are `&'static str`: text compiled
  into the program, shared without copying.
- **SQL injection is a type error.** sqlx 0.9 only accepts SQL written as a fixed string in the code. Our first
  version built SQL with `format!()` and the compiler refused it (`SqlSafeStr` is not implemented for
  `&String`), so every query is a literal and every value goes through `.bind()`.
- **Tests sit next to the code.** `#[cfg(test)] mod tests` at the bottom of `format.rs` and `pagination.rs`;
  `cargo test` runs them. The pagination test checks the exact links Laravel produced for page 20 of 40.

**Compared to Laravel + Inertia + Vue**

| Before                                                | Now                                                   |
| ----------------------------------------------------- | ----------------------------------------------------- |
| Controller returns `Inertia::render('News/Index', …)` | Handler fills a struct and renders `news/index.html`  |
| Props serialized to JSON, Vue renders in the browser  | The server sends finished HTML                        |
| `dayjs`, `Intl.NumberFormat` in the browser           | `src/format.rs` on the server (Skopje time zone)      |
| `v-reveal` directive, Vue `ref` for the menu          | `data-reveal` / `data-menu-*` + `public/js/app.js`    |
| Eloquent `$appends` (`image_url`)                     | A method on the struct, called from the template      |
| `->paginate(12)`                                      | `News::paginate(&db, page)` + `Paginator::links()`    |
| SSR via a Node process (optional)                     | Not needed: every response is already HTML            |

**Measured on this machine** (release build, SQLite): a **7.6 MB** binary, **~12 MB** of memory after 200
requests, **~0.5 ms** to render the home page. Laravel + PHP-FPM typically sits in the tens of milliseconds and
tens of megabytes per worker — but remember this site was never slow; the difference is real but not needed.

**What's intentionally not here yet**

- Sending the story form: CSRF protection, validation messages, the success message (sessions), rate limiting.
- Writing to the database at all, and migrations (`sqlx migrate`) to create the tables on a fresh database.
- The admin. Postgres for production. Deploying to Laravel Cloud.

**Exercises**

1. Make the news list show 6 posts per page instead of 12. Which file? (Hint: look for `12`.)
2. In `templates/pages/news/show.html`, rename `news.title` to `news.titel` and run `cargo build`. Read the
   error, then undo it.
3. Add a "Read time" to each news card: write `pub fn read_minutes(&self) -> usize` on `News` that counts the
   words in `format::strip_html(&self.content)` and divides by 200 (at least 1), then show it in
   `templates/components/cards.html`. Write a unit test for it.
4. `/news?page=abc` shows page 1 instead of an error. Find the code that decides that, and the test that
   proves it.
5. Harder: the home page runs its two queries with `tokio::try_join!`. Rewrite it with two `.await`s one after
   the other. Both work — why is the first one faster?

## What's next

- **Chapter 2 — Forms:** the story form end to end: sessions, CSRF tokens, validation with Macedonian messages,
  flash messages, rate limiting, writes, and SQL migrations.
- **Chapter 3 — The admin:** login with password hashing, protected routes, CRUD with image uploads, the
  rich-text editor.
- **Chapter 4 — Production:** Postgres, configuration for Laravel Cloud's Rust runtime, and a deploy.
