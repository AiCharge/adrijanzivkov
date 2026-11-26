//! Connecting to the database: SQLite on your machine, Postgres on Laravel Cloud, from one codebase.
//!
//! sqlx's `Any` driver picks the real driver from the URL (`sqlite:…` or `postgres://…`) at runtime.
//! The price is that `Any` only understands the basic types every database shares: integers, floats,
//! text, bools and bytes. So timestamps are read as text (`CAST(created_at AS TEXT)`) and parsed in
//! Rust, and written with SQL's own `CURRENT_TIMESTAMP`, which both databases understand.
//!
//! Queries use `$1, $2, …` placeholders: Postgres requires them and SQLite accepts them too.

use anyhow::Context;
use chrono::NaiveDateTime;
use sqlx::any::AnyPoolOptions;
use sqlx::{AnyPool, Executor};

/// The connection pool every query uses. A *type alias*: `Db` is just a shorter name for `AnyPool`.
pub type Db = AnyPool;

/// Opens the database and brings its tables up to date.
pub async fn connect(url: &str) -> anyhow::Result<Db> {
    // Registers the SQLite and Postgres drivers with `Any`. Needed once, before the first connection.
    sqlx::any::install_default_drivers();

    let is_postgres = url.starts_with("postgres");
    // An in-memory SQLite database exists once per connection, so tests must share a single one.
    let max_connections = if url.contains(":memory:") { 1 } else { 5 };

    let pool = AnyPoolOptions::new()
        .max_connections(max_connections)
        // Runs on every new connection. Postgres keeps times in the session's time zone, and the
        // timestamps here are UTC, as they were in Laravel.
        .after_connect(|connection, _meta| {
            Box::pin(async move {
                if connection
                    .backend_name()
                    .to_lowercase()
                    .contains("postgres")
                {
                    connection.execute("SET TIME ZONE 'UTC'").await?;
                }
                Ok(())
            })
        })
        .connect(url)
        .await
        .with_context(|| format!("could not connect to the database at `{}`", redact(url)))?;

    // `migrate!` reads the .sql files at compile time and builds them into the program, so a
    // deployment carries its own migrations. Each backend has its own folder because the SQL for
    // creating tables differs (`AUTOINCREMENT` vs `BIGSERIAL`).
    let migrator = if is_postgres {
        sqlx::migrate!("migrations/postgres")
    } else {
        sqlx::migrate!("migrations/sqlite")
    };
    migrator
        .run(&pool)
        .await
        .context("could not run database migrations")?;

    Ok(pool)
}

/// Parses a timestamp read as text. SQLite gives `2026-11-26 19:49:57`; Postgres may add fractions
/// of a second (`2026-11-26 19:49:57.123456`).
pub fn parse_timestamp(text: &str) -> Option<NaiveDateTime> {
    NaiveDateTime::parse_from_str(text, "%Y-%m-%d %H:%M:%S%.f")
        .or_else(|_| NaiveDateTime::parse_from_str(text, "%Y-%m-%dT%H:%M:%S%.f"))
        .ok()
}

/// Hides the password in a database URL before it goes into an error message.
fn redact(url: &str) -> String {
    match (url.find("://"), url.rfind('@')) {
        (Some(scheme_end), Some(at)) if at > scheme_end => {
            format!("{}://***{}", &url[..scheme_end], &url[at..])
        }
        _ => url.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_sqlite_and_postgres_timestamps() {
        assert!(parse_timestamp("2026-11-26 19:49:57").is_some());
        assert!(parse_timestamp("2026-11-26 19:49:57.123456").is_some());
        assert!(parse_timestamp("not a date").is_none());
    }

    #[test]
    fn hides_passwords_in_urls() {
        assert_eq!(
            redact("postgres://user:secret@host:5432/db"),
            "postgres://***@host:5432/db"
        );
        assert_eq!(
            redact("sqlite:database/database.sqlite"),
            "sqlite:database/database.sqlite"
        );
    }
}
