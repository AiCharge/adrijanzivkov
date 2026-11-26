use super::storage_url;
use crate::db::{self, Db};
use crate::format;
use crate::pagination::{Page, Paginator};

/// Something the foundation is collecting donations for.
#[derive(Debug, sqlx::FromRow)]
pub struct DonationNeed {
    pub id: i64,
    pub title: String,
    /// HTML from the admin's rich-text editor.
    pub description: String,
    /// HTML shown in the side box on the need's page, if any.
    pub sticky_content: Option<String>,
    pub image: String,
    pub needed_amount: i64,
    /// Set when the need was archived: it then no longer shows on the site.
    pub archived_at: Option<String>,
    pub created_at: Option<String>,
}

/// The fields the admin form sets.
pub struct DonationNeedInput<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub sticky_content: Option<&'a str>,
    pub needed_amount: i64,
}

/// The admin list's tabs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveFilter {
    All,
    Active,
    Archived,
}

impl ArchiveFilter {
    /// Reads `?archived=`; anything unknown shows all, as in Laravel.
    pub fn parse(value: &str) -> Self {
        match value {
            "active" => ArchiveFilter::Active,
            "archived" => ArchiveFilter::Archived,
            _ => ArchiveFilter::All,
        }
    }

    pub fn value(self) -> &'static str {
        match self {
            ArchiveFilter::All => "",
            ArchiveFilter::Active => "active",
            ArchiveFilter::Archived => "archived",
        }
    }
}

impl DonationNeed {
    pub fn image_url(&self) -> String {
        storage_url(&self.image)
    }

    /// e.g. `120.000`.
    pub fn amount(&self) -> String {
        format::amount(self.needed_amount)
    }

    /// The description as plain text, for card previews.
    pub fn summary(&self) -> String {
        format::strip_html(&self.description)
    }

    /// The side-box HTML, treating an empty editor (`""`) the same as no content.
    pub fn sticky_html(&self) -> Option<&str> {
        self.sticky_content
            .as_deref()
            .filter(|html| !html.trim().is_empty())
    }

    pub fn is_archived(&self) -> bool {
        self.archived_at.is_some()
    }

    /// e.g. `26.11.2026 20:49`, for the admin.
    pub fn created_at_label(&self) -> String {
        self.created_at
            .as_deref()
            .and_then(db::parse_timestamp)
            .map(format::date_time)
            .unwrap_or_default()
    }

    /// The newest needs still shown on the site, like `DonationNeed::notArchived()->latest()->take(6)`.
    pub async fn latest_active(db: &Db, limit: u32) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as(
            "SELECT id, title, description, sticky_content, image, needed_amount, \
             CAST(archived_at AS TEXT) AS archived_at, CAST(created_at AS TEXT) AS created_at \
             FROM donation_needs WHERE archived_at IS NULL ORDER BY created_at DESC, id DESC LIMIT $1",
        )
        .bind(i64::from(limit))
        .fetch_all(db)
        .await
    }

    /// One page of needs, newest first. The public list passes `ArchiveFilter::Active` and no search;
    /// the admin passes what its tabs and search box ask for.
    pub async fn paginate(
        db: &Db,
        page: Page,
        search: &str,
        filter: ArchiveFilter,
        path: &str,
    ) -> sqlx::Result<Paginator<Self>> {
        let pattern = format!("%{}%", search.to_lowercase());
        // The filter goes in as text: '' (all), 'active' or 'archived'.
        let total: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM donation_needs \
             WHERE ($1 = '' OR LOWER(title) LIKE $2) \
             AND ($3 = '' OR ($3 = 'active' AND archived_at IS NULL) OR ($3 = 'archived' AND archived_at IS NOT NULL))",
        )
        .bind(search)
        .bind(&pattern)
        .bind(filter.value())
        .fetch_one(db)
        .await?;

        let items = sqlx::query_as(
            "SELECT id, title, description, sticky_content, image, needed_amount, \
             CAST(archived_at AS TEXT) AS archived_at, CAST(created_at AS TEXT) AS created_at \
             FROM donation_needs \
             WHERE ($1 = '' OR LOWER(title) LIKE $2) \
             AND ($3 = '' OR ($3 = 'active' AND archived_at IS NULL) OR ($3 = 'archived' AND archived_at IS NOT NULL)) \
             ORDER BY created_at DESC, id DESC LIMIT $4 OFFSET $5",
        )
        .bind(search)
        .bind(&pattern)
        .bind(filter.value())
        .bind(i64::from(page.size))
        .bind(i64::from(page.offset()))
        .fetch_all(db)
        .await?;

        Ok(Paginator::new(items, total, page, path)
            .with_query(&[("search", search), ("archived", filter.value())]))
    }

    /// How many needs there are in total, active and archived, for the admin's tabs.
    pub async fn counts(db: &Db) -> sqlx::Result<(i64, i64, i64)> {
        let all: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM donation_needs")
            .fetch_one(db)
            .await?;
        let archived: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM donation_needs WHERE archived_at IS NOT NULL")
                .fetch_one(db)
                .await?;
        Ok((all, all - archived, archived))
    }

    /// The need at `/donation-needs/{id}`. Like the Laravel site, an archived need's page still opens.
    pub async fn find(db: &Db, id: i64) -> sqlx::Result<Option<Self>> {
        sqlx::query_as(
            "SELECT id, title, description, sticky_content, image, needed_amount, \
             CAST(archived_at AS TEXT) AS archived_at, CAST(created_at AS TEXT) AS created_at \
             FROM donation_needs WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(db)
        .await
    }

    pub async fn create(db: &Db, input: &DonationNeedInput<'_>, image: &str) -> sqlx::Result<()> {
        sqlx::query(
            "INSERT INTO donation_needs (title, description, sticky_content, image, needed_amount, created_at, updated_at) \
             VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
        )
        .bind(input.title)
        .bind(input.description)
        .bind(input.sticky_content)
        .bind(image)
        .bind(input.needed_amount)
        .execute(db)
        .await?;
        Ok(())
    }

    /// Changes a need; the image only when a new one was uploaded (`COALESCE` keeps the old one).
    pub async fn update(
        db: &Db,
        id: i64,
        input: &DonationNeedInput<'_>,
        image: Option<&str>,
    ) -> sqlx::Result<()> {
        sqlx::query(
            "UPDATE donation_needs SET title = $1, description = $2, sticky_content = $3, needed_amount = $4, \
             image = COALESCE($5, image), updated_at = CURRENT_TIMESTAMP WHERE id = $6",
        )
        .bind(input.title)
        .bind(input.description)
        .bind(input.sticky_content)
        .bind(input.needed_amount)
        .bind(image)
        .bind(id)
        .execute(db)
        .await?;
        Ok(())
    }

    /// Archives (hides from the site) or restores a need.
    pub async fn set_archived(db: &Db, id: i64, archived: bool) -> sqlx::Result<()> {
        let query = if archived {
            "UPDATE donation_needs SET archived_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE id = $1"
        } else {
            "UPDATE donation_needs SET archived_at = NULL, updated_at = CURRENT_TIMESTAMP WHERE id = $1"
        };
        sqlx::query(query).bind(id).execute(db).await?;
        Ok(())
    }
}
