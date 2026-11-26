//! Settings read from environment variables, like Laravel's `config/*.php` files reading `env()`.

use anyhow::Context;

/// Everything the server needs to start. Building it once at startup means a typo in a
/// variable stops the program immediately, instead of failing on the first request.
#[derive(Debug)]
pub struct Config {
    /// Where the database is, e.g. `sqlite:database/database.sqlite`.
    pub database_url: String,
    /// The port to listen on. Laravel Cloud sets `PORT`; locally it defaults to 3000.
    pub port: u16,
}

impl Config {
    /// Reads the settings, falling back to defaults that work for local development.
    pub fn from_env() -> anyhow::Result<Self> {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite:database/database.sqlite".to_string());

        // `match` must handle every case, so a missing variable and a bad value can't be forgotten.
        let port = match std::env::var("PORT") {
            Ok(value) => value
                .parse()
                .with_context(|| format!("PORT must be a number, got `{value}`"))?,
            Err(_) => 3000,
        };

        Ok(Self { database_url, port })
    }
}
