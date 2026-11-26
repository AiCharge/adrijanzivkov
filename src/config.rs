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
        // Laravel Cloud gives the database as `DB_URL`; `DATABASE_URL` is the usual Rust name.
        let database_url = std::env::var("DATABASE_URL")
            .or_else(|_| std::env::var("DB_URL"))
            .unwrap_or_else(|_| "sqlite:database/database.sqlite".to_string());

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
