//! Starts the web server. All the site itself lives in the library (`lib.rs`).

use anyhow::Context;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

use adrijanzivkov::config::Config;
use adrijanzivkov::models::User;
use adrijanzivkov::rate_limit::RateLimiter;
use adrijanzivkov::session::key_from_app_key;
use adrijanzivkov::state::AppState;
use adrijanzivkov::storage::Storage;
use axum_extra::extract::cookie::Key;

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

    let cookie_key = match &config.app_key {
        Some(app_key) => key_from_app_key(app_key).context("APP_KEY is not usable")?,
        None => {
            tracing::warn!(
                "APP_KEY is not set: using a random key, so logins end when the server restarts"
            );
            Key::generate()
        }
    };

    let db = adrijanzivkov::db::connect(&config.database_url).await?;

    // `cargo run -- create-admin <email> <name>` (or `adrijanzivkov create-admin …` on a server) adds an
    // admin account and stops, for a fresh database. The password comes from `ADMIN_PASSWORD`, so it
    // never ends up in the shell's history.
    let arguments: Vec<String> = std::env::args().collect();
    if arguments.get(1).map(String::as_str) == Some("create-admin") {
        let (Some(email), Some(name)) = (arguments.get(2), arguments.get(3)) else {
            anyhow::bail!(
                "usage: create-admin <email> <name>, with the password in ADMIN_PASSWORD"
            );
        };
        let password =
            std::env::var("ADMIN_PASSWORD").context("set ADMIN_PASSWORD to the new password")?;
        User::create(&db, name, email, &password).await?;
        println!("Created the admin account {email}.");
        return Ok(());
    }

    let state = AppState {
        db,
        cookie_key,
        rate_limiter: RateLimiter::default(),
        storage: Storage::from_env()?,
    };

    // `::` listens on every address, IPv6 and IPv4. Laravel Cloud requires it.
    let listener = TcpListener::bind(("::", config.port))
        .await
        .with_context(|| format!("could not listen on port {}", config.port))?;

    tracing::info!("listening on http://localhost:{}", config.port);

    // `into_make_service_with_connect_info` gives handlers the visitor's address, for rate limiting.
    axum::serve(
        listener,
        adrijanzivkov::app(state).into_make_service_with_connect_info::<SocketAddr>(),
    )
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
