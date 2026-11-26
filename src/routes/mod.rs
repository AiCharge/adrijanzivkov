//! Every URL the site answers, like `routes/web.php`, plus the static files.

mod about;
mod donation_needs;
mod home;
mod legal;
mod news;
mod story_submissions;

use askama::Template;
use axum::Router;
use axum::handler::HandlerWithoutStateExt;
use axum::response::Html;
use axum::routing::get;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

use crate::error::{AppError, AppResult};
use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    // Files in `public/` (CSS, JS, images, fonts, favicon) are served as they are. A path that is
    // neither a route nor a file gets the 404 page.
    let public_files = ServeDir::new("public").not_found_service(not_found.into_service());

    Router::new()
        .route("/", get(home::show))
        .route("/about", get(about::show))
        .route("/news", get(news::index))
        // `{slug}` captures that part of the path, like `{news:slug}` in Laravel's routes.
        .route("/news/{slug}", get(news::show))
        .route("/donation-needs", get(donation_needs::index))
        .route("/donation-needs/{id}", get(donation_needs::show))
        .route("/terms-of-use", get(legal::terms))
        .route("/privacy-policy", get(legal::privacy))
        .route("/story-submissions/create", get(story_submissions::create))
        .route("/up", get(|| async { "OK" }))
        // Uploaded images, which Laravel served through the `public/storage` link.
        .nest_service("/storage", ServeDir::new("storage/app/public"))
        .fallback_service(public_files)
        // Logs each request and how long it took, like Laravel's request logging.
        .layer(TraceLayer::new_for_http())
        // Hands `AppState` to every handler that asks for `State<AppState>`.
        .with_state(state)
}

/// Renders a page template into an HTML response.
///
/// `impl Template` accepts any type that implements the `Template` trait, which `#[derive(Template)]`
/// adds to each page struct. A *trait* is a shared set of methods, like a PHP interface.
fn render(page: &impl Template) -> AppResult<Html<String>> {
    Ok(Html(page.render()?))
}

async fn not_found() -> AppError {
    AppError::NotFound
}
