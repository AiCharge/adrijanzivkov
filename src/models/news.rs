use crate::db::{self, Db};

use super::storage_url;
use crate::format;
use crate::pagination::{Page, Paginator};

/// A news post. `#[derive(sqlx::FromRow)]` generates the code that fills these fields from a
/// query's columns by name, so a column missing from a `SELECT` is an error, not a silent `null`.
#[derive(Debug, sqlx::FromRow)]
pub struct News {
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
        // `query_as` maps each row to `Self`. `?` placeholders with `.bind()` keep values out of
        // the SQL text, as Eloquent's bindings do. sqlx 0.9 goes further: it only accepts SQL
        // written as a fixed string in the code (a `&'static str`), so text built at runtime,
        // which could contain user input, is refused at compile time.
        sqlx::query_as(
            "SELECT title, slug, short_description, main_image, content, CAST(created_at AS TEXT) AS created_at FROM news ORDER BY created_at DESC LIMIT $1",
        )
            .bind(i64::from(limit))
            .fetch_all(db)
            .await
    }

    /// One page of posts, newest first, like `News::latest()->paginate(12)`.
    pub async fn paginate(db: &Db, page: Page) -> sqlx::Result<Paginator<Self>> {
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM news")
            .fetch_one(db)
            .await?;

        let items = sqlx::query_as(
            "SELECT title, slug, short_description, main_image, content, CAST(created_at AS TEXT) AS created_at FROM news ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        )
        .bind(i64::from(page.size))
        .bind(i64::from(page.offset()))
        .fetch_all(db)
        .await?;

        Ok(Paginator::new(items, total, page, "/news"))
    }

    /// The post at `/news/{slug}`, or `None`, like route-model binding by slug.
    pub async fn find_by_slug(db: &Db, slug: &str) -> sqlx::Result<Option<Self>> {
        sqlx::query_as("SELECT title, slug, short_description, main_image, content, CAST(created_at AS TEXT) AS created_at FROM news WHERE slug = $1")
            .bind(slug)
            .fetch_optional(db)
            .await
    }
}
