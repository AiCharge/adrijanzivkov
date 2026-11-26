use super::storage_url;
use crate::db::{self, Db};
use crate::format;
use crate::pagination::{Page, Paginator};

/// A news post. `#[derive(sqlx::FromRow)]` generates the code that fills these fields from a
/// query's columns by name, so a column missing from a `SELECT` is an error, not a silent `null`.
#[derive(Debug, sqlx::FromRow)]
pub struct News {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub short_description: String,
    pub main_image: String,
    pub content: String,
    /// `Option` because the column allows NULL. Rust has no `null`: a value that may be missing
    /// is `Option<T>`, and the compiler makes you handle the `None` case before using it.
    /// Read as text (`CAST(… AS TEXT)`), because sqlx's `Any` driver has no date type (see db.rs).
    pub created_at: Option<String>,
}

/// The fields the admin form sets.
pub struct NewsInput<'a> {
    pub title: &'a str,
    pub short_description: &'a str,
    pub content: &'a str,
}

impl News {
    /// `&self` borrows the record for reading: the method can look at it but not change or keep it.
    pub fn image_url(&self) -> String {
        storage_url(&self.main_image)
    }

    /// e.g. `26 ноември 2026`.
    pub fn published_on(&self) -> String {
        self.created_at
            .as_deref()
            .and_then(db::parse_timestamp)
            .map(format::date_mk)
            .unwrap_or_default()
    }

    /// e.g. `26.11.2026 20:49`, for the admin.
    pub fn created_at_label(&self) -> String {
        self.created_at
            .as_deref()
            .and_then(db::parse_timestamp)
            .map(format::date_time)
            .unwrap_or_default()
    }

    /// e.g. `2026-11-26T19:49:57Z`, for the `<time datetime>` attribute.
    pub fn published_at_iso(&self) -> String {
        self.created_at
            .as_deref()
            .and_then(db::parse_timestamp)
            .map(|date| {
                date.and_utc()
                    .to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
            })
            .unwrap_or_default()
    }

    /// The newest posts, like `News::latest()->take($limit)->get()`.
    pub async fn latest(db: &Db, limit: u32) -> sqlx::Result<Vec<Self>> {
        // `query_as` maps each row to `Self`. `$1` placeholders with `.bind()` keep values out of the
        // SQL text, as Eloquent's bindings do. sqlx 0.9 goes further: it only accepts SQL written as a
        // fixed string in the code (a `&'static str`), so text built at runtime, which could contain
        // user input, is refused at compile time.
        sqlx::query_as(
            "SELECT id, title, slug, short_description, main_image, content, CAST(created_at AS TEXT) AS created_at \
             FROM news ORDER BY created_at DESC, id DESC LIMIT $1",
        )
        .bind(i64::from(limit))
        .fetch_all(db)
        .await
    }

    /// One page of posts, newest first, optionally filtered by a search in the title or description.
    /// The public list (`/news`) passes an empty search.
    pub async fn paginate(
        db: &Db,
        page: Page,
        search: &str,
        path: &str,
    ) -> sqlx::Result<Paginator<Self>> {
        // An empty search matches everything: `$1 = ''` is true when there's nothing to look for.
        let pattern = format!("%{}%", search.to_lowercase());

        let total: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM news \
             WHERE ($1 = '' OR LOWER(title) LIKE $2 OR LOWER(short_description) LIKE $2)",
        )
        .bind(search)
        .bind(&pattern)
        .fetch_one(db)
        .await?;

        let items = sqlx::query_as(
            "SELECT id, title, slug, short_description, main_image, content, CAST(created_at AS TEXT) AS created_at \
             FROM news WHERE ($1 = '' OR LOWER(title) LIKE $2 OR LOWER(short_description) LIKE $2) \
             ORDER BY created_at DESC, id DESC LIMIT $3 OFFSET $4",
        )
        .bind(search)
        .bind(&pattern)
        .bind(i64::from(page.size))
        .bind(i64::from(page.offset()))
        .fetch_all(db)
        .await?;

        Ok(Paginator::new(items, total, page, path).with_query(&[("search", search)]))
    }

    /// The post at `/news/{slug}`, or `None`, like route-model binding by slug.
    pub async fn find_by_slug(db: &Db, slug: &str) -> sqlx::Result<Option<Self>> {
        sqlx::query_as(
            "SELECT id, title, slug, short_description, main_image, content, CAST(created_at AS TEXT) AS created_at \
             FROM news WHERE slug = $1",
        )
        .bind(slug)
        .fetch_optional(db)
        .await
    }

    /// Saves a new post and returns its slug, made from the title and kept unique (`ista-vest`,
    /// `ista-vest-2`, …), as the Laravel model did.
    pub async fn create(db: &Db, input: &NewsInput<'_>, main_image: &str) -> sqlx::Result<String> {
        let slug = Self::unique_slug(db, input.title).await?;

        sqlx::query(
            "INSERT INTO news (title, slug, short_description, main_image, content, created_at, updated_at) \
             VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
        )
        .bind(input.title)
        .bind(&slug)
        .bind(input.short_description)
        .bind(main_image)
        .bind(input.content)
        .execute(db)
        .await?;

        Ok(slug)
    }

    /// Changes a post. The slug stays, so links to it keep working; the image changes only when a
    /// new one was uploaded.
    pub async fn update(
        db: &Db,
        id: i64,
        input: &NewsInput<'_>,
        main_image: Option<&str>,
    ) -> sqlx::Result<()> {
        sqlx::query(
            "UPDATE news SET title = $1, short_description = $2, content = $3, \
             main_image = COALESCE($4, main_image), updated_at = CURRENT_TIMESTAMP WHERE id = $5",
        )
        .bind(input.title)
        .bind(input.short_description)
        .bind(input.content)
        .bind(main_image)
        .bind(id)
        .execute(db)
        .await?;

        Ok(())
    }

    async fn unique_slug(db: &Db, title: &str) -> sqlx::Result<String> {
        let base = format::slug(title);
        let base = if base.is_empty() {
            "vest".to_string()
        } else {
            base
        };
        let mut slug = base.clone();
        let mut suffix = 2;

        loop {
            let taken: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM news WHERE slug = $1")
                .bind(&slug)
                .fetch_one(db)
                .await?;
            if taken == 0 {
                return Ok(slug);
            }
            slug = format!("{base}-{suffix}");
            suffix += 1;
        }
    }
}
