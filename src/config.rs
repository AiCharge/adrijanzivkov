//! Settings read from environment variables, like Laravel's `config/*.php` files reading `env()`.

use anyhow::Context;

/// Everything the server needs to start. Building it once at startup means a typo in a
/// variable stops the program immediately, instead of failing on the first request.
#[derive(Debug)]
pub struct Config {
    /// Where the database is: `sqlite:database/database.sqlite` locally, `postgres://…` on Laravel Cloud.
    pub database_url: String,
    /// The port to listen on. Laravel Cloud sets `PORT`; locally it defaults to 3000.
    pub port: u16,
    /// The secret that encrypts session cookies. Laravel's `APP_KEY` works as is.
    pub app_key: Option<String>,
}

impl Config {
    /// Reads the settings, falling back to defaults that work for local development.
    pub fn from_env() -> anyhow::Result<Self> {
        // `DATABASE_URL` is the usual Rust name; Laravel names it `DB_URL`, or gives the pieces as
        // `DB_HOST`, `DB_DATABASE`… (see `url_from_parts`). Without any, the local SQLite file.
        let database_url = std::env::var("DATABASE_URL")
            .or_else(|_| std::env::var("DB_URL"))
            .ok()
            .filter(|url| !url.is_empty())
            .or_else(url_from_parts)
            .unwrap_or_else(|| "sqlite:database/database.sqlite".to_string());

        // `match` must handle every case, so a missing variable and a bad value can't be forgotten.
        let port = match std::env::var("PORT") {
            Ok(value) => value
                .parse()
                .with_context(|| format!("PORT must be a number, got `{value}`"))?,
            Err(_) => 3000,
        };

        let app_key = std::env::var("APP_KEY").ok().filter(|key| !key.is_empty());

        Ok(Self {
            database_url,
            port,
            app_key,
        })
    }
}

/// A Postgres URL from Laravel's separate variables (`DB_CONNECTION=pgsql`, `DB_HOST`, `DB_PORT`,
/// `DB_DATABASE`, `DB_USERNAME`, `DB_PASSWORD`, and `DB_SSLMODE`, e.g. `require` for a managed database).
/// `None` unless the connection is Postgres and a host is given.
fn url_from_parts() -> Option<String> {
    let variable = |name: &str| std::env::var(name).ok().filter(|value| !value.is_empty());

    if variable("DB_CONNECTION")? != "pgsql" {
        return None;
    }

    let host = variable("DB_HOST")?;
    let port = variable("DB_PORT").unwrap_or_else(|| "5432".to_string());
    let database = variable("DB_DATABASE").unwrap_or_else(|| "postgres".to_string());
    let username = variable("DB_USERNAME").unwrap_or_else(|| "postgres".to_string());
    let password = variable("DB_PASSWORD").unwrap_or_default();
    let ssl_mode = variable("DB_SSLMODE").unwrap_or_else(|| "prefer".to_string());

    // Passwords can contain `@`, `/` or `:`, which mean something in a URL, so they're percent-encoded.
    Some(format!(
        "postgres://{}:{}@{host}:{port}/{database}?sslmode={ssl_mode}",
        crate::pagination::url_encode(&username),
        crate::pagination::url_encode(&password),
    ))
}
