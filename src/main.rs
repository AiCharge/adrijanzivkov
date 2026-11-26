//! The website of the Adrijan Zhivkov Foundation, served by a single Rust program.
//!
//! Reading order for learning: `main.rs` (this file) → `config.rs` → `state.rs` → `routes/mod.rs`
//! → one handler such as `routes/news.rs` → `models/news.rs` → `templates/pages/news/index.html`.

// `mod` declares a module. Each one maps to a file (`config.rs`) or a folder with a `mod.rs`
// (`models/mod.rs`). Nothing is loaded automatically like Composer's autoloader: the compiler
// only sees the files you list here.
mod config;
mod error;
mod format;
mod models;
mod pagination;
mod routes;
mod site;
mod state;

use anyhow::Context;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

use crate::config::Config;
use crate::state::AppState;

// `#[tokio::main]` turns `main` into an async function by starting Tokio, the async runtime.
// Rust has `async`/`await` in the language, but no built-in runtime to run it: Tokio is the
// event loop that plays the role PHP-FPM or Octane plays for Laravel.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // `.ok()` turns the `Result` into an `Option` and drops it: a missing `.env` file is fine,
    // the environment may already hold the values (as it does on Laravel Cloud).
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("adrijanzivkov=info,tower_http=info")),
        )
        .init();

    // The `?` operator: if `from_env()` returns `Err`, return that error from `main` right away;
    // if it returns `Ok(config)`, unwrap the value and carry on. It replaces try/catch.
    let config = Config::from_env()?;

    // Read-only for now: until the forms are ported, this server can't change the data.
    let connect_options = SqliteConnectOptions::from_str(&config.database_url)
        .with_context(|| format!("invalid DATABASE_URL `{}`", config.database_url))?
        .read_only(true);

    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await
        .context("could not open the database")?;

    let state = AppState { db };
    let app = routes::router(state);

    // `::` listens on every address, IPv6 and IPv4. Laravel Cloud requires it.
    let listener = TcpListener::bind(("::", config.port))
        .await
        .with_context(|| format!("could not listen on port {}", config.port))?;

    tracing::info!("listening on http://localhost:{}", config.port);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

/// Waits for Ctrl+C locally, or SIGTERM from the platform, so requests in flight can finish
/// before the program stops. Laravel Cloud sends SIGTERM before replacing an instance.
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to listen for Ctrl+C");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to listen for SIGTERM")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    // `select!` waits for whichever future finishes first.
    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }

    tracing::info!("shutting down");
}
