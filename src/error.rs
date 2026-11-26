//! How failures become HTTP responses, like Laravel's exception handler.
//!
//! Rust has no exceptions. A function that can fail returns `Result<T, E>`: either `Ok(value)` or
//! `Err(error)`. Handlers here return `AppResult<T>`, and `?` passes any error up to axum, which
//! turns it into a response through the `IntoResponse` implementation below.

use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};

/// The two ways a request can fail that the visitor should know about.
#[derive(Debug)]
pub enum AppError {
    /// The page or record doesn't exist: shows the 404 page.
    NotFound,
    /// Anything unexpected, such as a database error: logged, then shown as a 500.
    Internal(anyhow::Error),
}

/// A shorthand used by every handler: `AppResult<Html<String>>` instead of the full `Result`.
pub type AppResult<T> = Result<T, AppError>;

#[derive(Template)]
#[template(path = "errors/404.html")]
struct NotFoundPage;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound => {
                let body = NotFoundPage
                    .render()
                    .unwrap_or_else(|_| "Страницата не е пронајдена.".to_string());
                (StatusCode::NOT_FOUND, Html(body)).into_response()
            }
            AppError::Internal(error) => {
                // `{error:#}` prints the error with its whole chain of causes.
                tracing::error!("{error:#}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Настана грешка на серверот.",
                )
                    .into_response()
            }
        }
    }
}

/// Lets `?` convert any common error (sqlx, Askama, I/O) into `AppError::Internal`.
///
/// This is a *blanket implementation*: one `impl` that covers every type `E` which can become an
/// `anyhow::Error`. Laravel gets the same effect from all exceptions sharing a base class.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(error: E) -> Self {
        AppError::Internal(error.into())
    }
}
