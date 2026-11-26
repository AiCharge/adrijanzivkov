use sqlx::SqlitePool;

use super::storage_url;
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

    /// The newest needs still shown on the site, like `DonationNeed::notArchived()->latest()->take(6)`.
    pub async fn latest_active(db: &SqlitePool, limit: u32) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as(
            "SELECT id, title, description, sticky_content, image, needed_amount FROM donation_needs WHERE archived_at IS NULL ORDER BY created_at DESC LIMIT ?",
        )
        .bind(limit)
        .fetch_all(db)
        .await
    }

    /// One page of the needs still shown on the site, newest first.
    pub async fn paginate_active(db: &SqlitePool, page: Page) -> sqlx::Result<Paginator<Self>> {
        let total: u32 =
            sqlx::query_scalar("SELECT COUNT(*) FROM donation_needs WHERE archived_at IS NULL")
                .fetch_one(db)
                .await?;

        let items = sqlx::query_as(
            "SELECT id, title, description, sticky_content, image, needed_amount FROM donation_needs WHERE archived_at IS NULL ORDER BY created_at DESC LIMIT ? OFFSET ?",
        )
        .bind(page.size)
        .bind(page.offset())
        .fetch_all(db)
        .await?;

        Ok(Paginator::new(items, total, page, "/donation-needs"))
    }

    /// The need at `/donation-needs/{id}`. Like the Laravel site, an archived need's page still opens.
    pub async fn find(db: &SqlitePool, id: i64) -> sqlx::Result<Option<Self>> {
        sqlx::query_as("SELECT id, title, description, sticky_content, image, needed_amount FROM donation_needs WHERE id = ?")
            .bind(id)
            .fetch_optional(db)
            .await
    }
}
