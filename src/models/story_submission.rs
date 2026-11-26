use crate::db::{self, Db};
use crate::format;
use crate::pagination::{Page, Paginator};

/// Where a submitted story is in the foundation's handling of it.
///
/// A Rust `enum` lists every possible value, and `match` on it must handle all of them, so adding a
/// status without deciding its label and colour is a compile error. It's stored in the database as
/// text (`new`, `accepted`…), exactly as Laravel's backed enum stored it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    New,
    Accepted,
    Rejected,
    Completed,
}

impl Status {
    /// Every status, in the order the admin lists them.
    pub const ALL: [Status; 4] = [
        Status::New,
        Status::Accepted,
        Status::Rejected,
        Status::Completed,
    ];

    /// The value stored in the database and used in URLs.
    pub fn value(self) -> &'static str {
        match self {
            Status::New => "new",
            Status::Accepted => "accepted",
            Status::Rejected => "rejected",
            Status::Completed => "completed",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Status::New => "Ново",
            Status::Accepted => "Прифатено",
            Status::Rejected => "Одбиено",
            Status::Completed => "Завршено",
        }
    }

    /// The badge colour in the admin (see the `badge` macro).
    pub fn color(self) -> &'static str {
        match self {
            Status::New => "info",
            Status::Accepted => "success",
            Status::Rejected => "danger",
            Status::Completed => "neutral",
        }
    }

    /// The status for a stored or submitted value; `None` for anything else.
    pub fn parse(value: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|status| status.value() == value)
    }
}

/// A story sent through the public form.
#[derive(Debug, sqlx::FromRow)]
pub struct StorySubmission {
    pub id: i64,
    pub name: String,
    pub phone: String,
    pub message: String,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: Option<String>,
}

impl StorySubmission {
    /// The status as the enum. An unknown stored value shows as "new" rather than failing the page.
    pub fn status(&self) -> Status {
        Status::parse(&self.status).unwrap_or(Status::New)
    }

    /// e.g. `26.11.2026 20:49`.
    pub fn received_at(&self) -> String {
        self.created_at
            .as_deref()
            .and_then(db::parse_timestamp)
            .map(format::date_time)
            .unwrap_or_default()
    }

    /// Saves a new story with the status "new", like `StorySubmission::create(…)`.
    pub async fn create(db: &Db, name: &str, phone: &str, message: &str) -> sqlx::Result<()> {
        sqlx::query(
            "INSERT INTO story_submissions (name, phone, message, status, created_at, updated_at) \
             VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
        )
        .bind(name)
        .bind(phone)
        .bind(message)
        .bind(Status::New.value())
        .execute(db)
        .await?;

        Ok(())
    }

    /// One page for the admin list, newest first, optionally filtered by name and status.
    pub async fn paginate(
        db: &Db,
        page: Page,
        search: &str,
        status: Option<Status>,
    ) -> sqlx::Result<Paginator<Self>> {
        // An empty filter matches everything: `$1 = ''` is true when there's no search.
        let pattern = format!("%{}%", search.to_lowercase());
        let status_value = status.map(Status::value).unwrap_or("");

        let total: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM story_submissions \
             WHERE ($1 = '' OR LOWER(name) LIKE $2) AND ($3 = '' OR status = $3)",
        )
        .bind(search)
        .bind(&pattern)
        .bind(status_value)
        .fetch_one(db)
        .await?;

        let items = sqlx::query_as(
            "SELECT id, name, phone, message, status, notes, CAST(created_at AS TEXT) AS created_at \
             FROM story_submissions \
             WHERE ($1 = '' OR LOWER(name) LIKE $2) AND ($3 = '' OR status = $3) \
             ORDER BY created_at DESC, id DESC LIMIT $4 OFFSET $5",
        )
        .bind(search)
        .bind(&pattern)
        .bind(status_value)
        .bind(i64::from(page.size))
        .bind(i64::from(page.offset()))
        .fetch_all(db)
        .await?;

        Ok(
            Paginator::new(items, total, page, "/admin/story-submissions")
                .with_query(&[("search", search), ("status", status_value)]),
        )
    }

    /// How many stories have each status, plus `all`, for the admin's filter tabs.
    pub async fn counts(db: &Db) -> sqlx::Result<Vec<(String, i64)>> {
        let mut counts = vec![(
            "all".to_string(),
            sqlx::query_scalar("SELECT COUNT(*) FROM story_submissions")
                .fetch_one(db)
                .await?,
        )];

        for status in Status::ALL {
            let count: i64 =
                sqlx::query_scalar("SELECT COUNT(*) FROM story_submissions WHERE status = $1")
                    .bind(status.value())
                    .fetch_one(db)
                    .await?;
            counts.push((status.value().to_string(), count));
        }

        Ok(counts)
    }

    pub async fn find(db: &Db, id: i64) -> sqlx::Result<Option<Self>> {
        sqlx::query_as(
            "SELECT id, name, phone, message, status, notes, CAST(created_at AS TEXT) AS created_at \
             FROM story_submissions WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(db)
        .await
    }

    /// Changes only the status and notes: name, phone and message stay as the visitor sent them.
    pub async fn update(db: &Db, id: i64, status: Status, notes: Option<&str>) -> sqlx::Result<()> {
        sqlx::query(
            "UPDATE story_submissions SET status = $1, notes = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3",
        )
        .bind(status.value())
        .bind(notes)
        .bind(id)
        .execute(db)
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn statuses_round_trip_through_their_stored_value() {
        for status in Status::ALL {
            assert_eq!(Status::parse(status.value()), Some(status));
        }
        assert_eq!(Status::parse("bogus"), None);
    }
}
