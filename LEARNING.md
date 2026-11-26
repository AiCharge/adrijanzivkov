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
pnpm run build        # builds the site and admin CSS (Tailwind) and the admin's editor script (esbuild)
cargo run             # builds, migrates the database and starts the server on http://localhost:3000
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

---

## Chapter 2 — Forms: sessions, CSRF, validation, rate limiting, writes

**What changed.** The "share your story" form works end to end: it checks a CSRF token, validates with
the Macedonian messages from Laravel's `StoreStorySubmissionRequest`, keeps what was typed when there are
errors, saves the story, redirects with a one-time thank-you message, and allows 4 stories a minute per
visitor. The database is now writable, created and updated by **migrations**, and the same code runs on
**SQLite locally and Postgres in production**. The project became a *library* plus a small `main.rs`, so
integration tests can call the app directly.

**Read, in this order**

1. `src/db.rs` — one connection type for both databases, and the migrations in `migrations/`.
2. `src/session.rs` — the `Session` extractor: encrypted cookies for the CSRF token, flash messages and
   (in Chapter 3) the admin login.
3. `src/validation.rs` — the small `Errors` type that replaces Form Requests.
4. `src/rate_limit.rs` — `Arc<Mutex<HashMap<…>>>`, Rust's safe shared state.
5. `src/routes/story_submissions.rs` — the two handlers, `create` (GET) and `store` (POST).
6. `templates/pages/story_submissions/create.html` — errors, kept values, the thank-you message.
7. `src/models/story_submission.rs` — the `Status` enum and the `INSERT`.
8. `tests/common/mod.rs` → `tests/story_form.rs` — the test client and the form's tests.

**Rust ideas**

- **Your own extractor that also writes the response.** `Session` implements `FromRequestParts` (read the
  cookies) *and* `IntoResponseParts` (write `Set-Cookie`). Handlers return `(session, page)` and axum
  combines them. Nothing is hidden in global state: if a handler changes the session, it must return it.
- **`&self` vs `&mut self`.** `session.verify_csrf(…)` only reads (`&self`); `session.flash(…)` changes the
  session, so it takes `&mut self`, and the handler must declare `mut session`. The compiler tracks this.
- **Shared mutable state needs `Arc<Mutex<…>>`.** Requests run on several threads at once. The compiler
  refuses to let two threads touch a plain `HashMap` (that would be a data race); wrapping it in a `Mutex`
  (one thread at a time) inside an `Arc` (shared ownership) is how you say "this is shared, safely".
- **Enums with behaviour.** `Status` replaces Laravel's backed enum. `match` must list every variant, so
  adding a status without a label or colour doesn't compile.
- **`serde` turns form bodies into structs.** `Form<StoryForm>` parses `name=…&phone=…` into a typed struct.
  `#[serde(default)]` makes missing fields empty instead of rejecting the request.
- **One code path, two databases.** sqlx's `Any` driver only knows types every database shares, so dates are
  read as text (`CAST(created_at AS TEXT)`) and written with `CURRENT_TIMESTAMP`. Placeholders are `$1, $2`.
- **Lifetimes, again.** The test helper `sql(&self, query: &'static str)` must promise the SQL is fixed text
  in the program, because sqlx refuses SQL built at runtime. The compiler explained this when it was `&str`.

**Compared to Laravel**

| Laravel                                   | Here                                                         |
| ----------------------------------------- | ------------------------------------------------------------ |
| Sessions in the `sessions` table          | Encrypted cookies (`PrivateCookieJar`), keyed from `APP_KEY` |
| `VerifyCsrfToken` middleware, `@csrf`     | `session.csrf_token()` in the form, `verify_csrf` in `store` |
| Form Request + `$errors` in Blade         | `StoryForm::validate()` returning `Errors`                   |
| `->with('success', …)` / `session('…')`   | `session.flash(…)` / `session.take_flash()`                  |
| `throttle:4,1` route middleware           | `state.rate_limiter.hit(…)` inside the handler               |
| `php artisan migrate`                     | `sqlx::migrate!` runs on every start, before serving         |
| `RefreshDatabase` + `$this->post(…)`      | `TestClient::new()` (in-memory SQLite) + `post_form(…)`      |

**Security notes worth reading in the code**

- The CSRF comparison is constant-time (`constant_time_eq`), so its timing reveals nothing.
- Behind a load balancer the visitor's IP is the **last** `X-Forwarded-For` entry; the first can be faked
  (`site.rs`, `ClientIp`).
- The rate limiter lives in memory: it resets on restart and each server counts separately. Good enough for
  one instance; with several, the counts would move to the database.

**Exercises**

1. Submit the form with the "agree" box unticked and read the 422 response. Then find the line that decides
   a checkbox counts as ticked (`is_accepted`).
2. Change the limit to 2 stories a minute and run `cargo test`. Which test fails, and why?
3. Add a rule: the phone number may only contain digits, spaces, `+`, `-` and `/`. Write the check in
   `StoryForm::validate`, a Macedonian message, and a test in `tests/story_form.rs`.
4. Run the server against Postgres:
   `DATABASE_URL=postgres://user@localhost/adrijan cargo run` — the migrations create the tables on start.

---

## Chapter 3 — The admin: login, guarded pages, uploads, the editor

**What changed.** The whole admin from `main` is back, server-rendered: login with "remember me" and
lockout, the green sidebar layout with breadcrumbs, a light/dark/system theme and a foldable sidebar,
donation needs (list with tabs, search and pages, create, edit, archive and restore with a confirmation
dialog), news (list, search, create, edit, unique slugs), story submissions (status tabs, status and notes),
the profile page, image uploads, and the Tiptap rich-text editor with image uploads inside the text.

**Read, in this order**

1. `src/routes/admin/mod.rs` — the routes, and the `Admin` extractor that guards every page.
2. `src/routes/admin/auth.rs` + `src/models/user.rs` — logging in: bcrypt, lockout, timing-safe failures.
3. `src/routes/admin/news.rs` — a full CRUD section: list, form, multipart save shared by create and edit.
4. `src/upload.rs` — reading `multipart/form-data` and recognising images by their bytes.
5. `src/storage.rs` — where files go, and `OnceLock` for a value set once at startup.
6. `templates/admin/layouts/app.html` → `templates/admin/components/ui.html` → `templates/admin/pages/news/form.html`.
7. `assets/css/admin.css` — the shadcn component classes as `ui-*` CSS classes.
8. `public/js/admin.js` and `assets/js/editor.js` — the only JavaScript, and why the editor needs a bundler.
9. `tests/admin.rs` — every admin behaviour, including multipart uploads in tests.

**Rust ideas**

- **An extractor as the auth middleware.** `Admin` implements `FromRequestParts` with
  `type Rejection = Response`. If there's no valid login, axum returns that redirect instead of calling the
  handler. A handler that takes `admin: Admin` *cannot* run for a guest — the type system enforces it, and
  there's no route list to forget a middleware on.
- **Blocking work in an async program.** bcrypt takes ~250 ms of CPU on purpose. `spawn_blocking` moves it
  to a thread pool meant for that, so the async threads keep serving other visitors meanwhile.
- **`LazyLock` and `OnceLock`.** Values computed or set once and then shared by every thread: the dummy
  password hash (`user.rs`) and the storage URL (`storage.rs`).
- **`let … else`.** `let (true, Some(amount)) = (errors.is_empty(), amount) else { return …; };` either binds
  the value or leaves the function — no nested `if`s.
- **Slice patterns and match guards.** `ImageKind::detect` matches file signatures with `[0xFF, 0xD8, 0xFF, ..]`
  and `_ if bytes.starts_with(b"RIFF") && …`.
- **Ownership moving out.** `form.take_file("image")` *removes* the bytes from the form and hands them over,
  so a 10 MB upload is never copied.
- **Templates are type-checked too.** A `{% call ui::rich_editor(…) %}` with a wrong argument, or a field the
  page struct doesn't have, fails `cargo build`. One limit found the hard way: inside a Rust `format!` in a
  template, Askama only sees *local* names, so `edit.html` copies a field into a `{% let %}` first.

**Compared to Laravel + Inertia + Vue**

| Before                                              | Now                                                             |
| --------------------------------------------------- | --------------------------------------------------------------- |
| `auth` middleware, `redirect()->intended()`         | `Admin` extractor; the intended page kept in an encrypted cookie |
| `LoginRequest` lockout via `RateLimiter`            | Same idea: `rate_limiter.too_many(…)` keyed by email and IP     |
| `Hash::check` / `Hash::make`                         | `bcrypt` in `spawn_blocking`; hashes stay `$2y$`, so both read them |
| `$request->file('image')->store('news')`            | `MultipartForm` + `check_image` + `storage.put("news", …)`      |
| shadcn-vue components (Button, Card, Table…)        | the same classes as `ui-*` CSS, and Askama macros in `ui.html`   |
| Reka UI dialogs, dropdowns                          | the browser's own `<dialog>` and `<details>` elements           |
| Tiptap inside a Vue component                       | Tiptap bundled by esbuild, filling a hidden input                |
| Inertia `useForm` keeps typed values on error       | the handler renders the form again with the values it received  |

**One thing that's simpler than before:** every admin page is a normal HTML page. A form posts, the server
redirects or shows the errors, and the browser's back button, reload and "open in new tab" all just work.

**What changed on purpose**

- Forms post to `POST …/{id}` instead of `PUT`/`PATCH`: HTML forms can only send GET and POST.
- After a failed upload validation the image must be picked again: browsers don't let a page refill a file input.
- The admin's password rule is always the strict production one (12+ characters, mixed case, digit, symbol).

**Exercises**

1. Remove `admin: Admin` from `news::index` and run `cargo test`. Which test fails, and why is that the
   behaviour you want?
2. Add a "Delete" button for news posts: a `POST /admin/news/{slug}/delete` route, a `News::delete` query,
   a `confirm_dialog` in `form.html`, and a test.
3. Show how many stories are "new" next to "Поднесени приказни" in the sidebar. Where would that number come
   from, and which struct has to carry it to `layouts/app.html`?
4. Make a Postgres-only bug impossible to miss: run `cargo test` with `DATABASE_URL` pointing at Postgres.
   (Hint: the test client uses `sqlite::memory:` — what would it take to read it from the environment?)

---

## Chapter 4 — Production: Postgres, object storage, one self-contained binary

**What changed.** The program is ready for Laravel Cloud's Rust runtime:

- **Everything is inside the binary.** `public/` (CSS, scripts, fonts, images) is embedded with `rust-embed`,
  the SQL migrations with `sqlx::migrate!`, the templates by Askama. A deploy is one file.
- **Postgres** is configured from `DATABASE_URL`, `DB_URL`, or Laravel's own `DB_CONNECTION=pgsql`, `DB_HOST`,
  `DB_PORT`, `DB_DATABASE`, `DB_USERNAME`, `DB_PASSWORD` and `DB_SSLMODE` (TLS through rustls).
- **Uploads go to the bucket** when `FILESYSTEM_DISK=s3`, using Laravel's `AWS_*` variables, so an existing
  Laravel Cloud bucket keeps working and old image paths still resolve against `AWS_URL`.
- **Cache-safe assets.** Pages link to `/css/app.css?v=<fingerprint>`; those are cached for a year and change
  name on every deploy. Every file has an ETag, so unchanged files answer `304 Not Modified`.
- **`create-admin`** adds an admin account to a fresh database: `ADMIN_PASSWORD=… adrijanzivkov create-admin <email> <name>`.
- **CI** (`.github/workflows/ci.yml`) runs `cargo fmt --check`, `clippy -D warnings`, the tests, and checks
  the committed CSS/JS match their sources.

**Read, in this order**

1. `src/assets.rs` — embedding, content types, ETags, fingerprinted URLs.
2. `src/storage.rs` — an `enum` whose variants carry different data (`Local { root }`, `S3(Arc<AmazonS3>)`).
3. `src/config.rs` — building a database URL from Laravel's variables, with percent-encoding.
4. `src/main.rs` — the `create-admin` command next to the server.
5. `.github/workflows/ci.yml`.

**Rust ideas**

- **Compile-time vs runtime.** `#[derive(Embed)]`, `sqlx::migrate!` and `#[derive(Template)]` all read files
  *while compiling*. A missing template or migration is a build error, not a production surprise.
- **Enums with data.** `Storage::Local { root }` and `Storage::S3(client)` are one type; `match` forces each
  method to handle both, so you can't add S3 and forget it in `put`.
- **Features.** `Cargo.toml` turns on only what's used: sqlx's `any`, `sqlite`, `postgres`, `migrate` and
  `tls-rustls`; rust-embed's `include-exclude`. Smaller builds, fewer dependencies to trust.

**Deploying to Laravel Cloud** (not yet tried against a real environment — expect to adjust)

1. Create a **separate application** (or environment) on this branch, so the Laravel site keeps running.
   Cloud detects the `Cargo.toml` at the repository root and builds with `cargo build --release`.
2. Start command: `adrijanzivkov`. It listens on `::` and Cloud's `$PORT`, and stops cleanly on SIGTERM.
   Health check: `/up`.
3. Environment: `APP_KEY` (copy Laravel's so the secret stays the same), attach the database (Cloud provides
   the `DB_*` variables; set `DB_SSLMODE=require` if it doesn't), attach the bucket and set
   `FILESYSTEM_DISK=s3`. Logs: `RUST_LOG=adrijanzivkov=info,tower_http=info`.
4. On first start the migrations run. They use `CREATE TABLE IF NOT EXISTS`, so the tables Laravel created
   are kept with their data; sqlx adds its own `_sqlx_migrations` table next to Laravel's `migrations` table.
5. Existing admins log in with their Laravel passwords (the `$2y$` bcrypt hashes are read as they are).

**What's different from Laravel in production**

- Sessions are encrypted cookies, so there's no `sessions` table, and logging out ends the session in that
  browser only. A changed password doesn't sign out other browsers.
- Rate limits live in memory per instance (see Chapter 2).
- Laravel Cloud has no managed queues or scheduled tasks for Rust; this site needs neither.

**Measured locally** (release build, SQLite, no other load): the binary is **27 MB** with the embedded fonts,
images and the S3 client; it serves a page in **~0.5–0.6 ms** and uses **~14 MB** of memory after 400 requests.

**Exercises**

1. Run `cargo build --release`, copy `target/release/adrijanzivkov` to an empty folder with only a `database/`
   folder, and run it there. Which files does it still need from the repository? (Answer: none but the database.)
2. Change a colour in `assets/css/app.css`, run `pnpm run build`, reload a page and look at the CSS link's `?v=`.
3. Read `config.rs` and write a unit test for `url_from_parts` with a password that contains `@`.
## What's next

The migration is complete: everything the Laravel site did, this program does. Ideas for going further, in
rising difficulty:

- Write the exercises' features (news deletion, a "new stories" counter, phone validation).
- Switch the queries to `sqlx::query_as!`, which checks SQL against a real database at compile time.
- Run the integration tests against Postgres in CI with a service container.
- Deploy to a Laravel Cloud environment and compare response times and memory with the Laravel app.
