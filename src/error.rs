//! How failures become HTTP responses, like Laravel's exception handler.
//!
//! Rust has no exceptions. A function that can fail returns `Result<T, E>`: either `Ok(value)` or
//! `Err(error)`. Handlers here return `AppResult<T>`, and `?` passes any error up to axum, which
//! turns it into a response through the `IntoResponse` implementation below.

use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use std::time::Duration;

/// The ways a request can fail that the visitor should know about.
#[derive(Debug)]
pub enum AppError {
    /// The page or record doesn't exist: 404.
    NotFound,
    /// A form came back without a valid CSRF token (often a page left open too long): 419.
    PageExpired,
    /// Too many tries in a short time; the value says how long to wait: 429.
    TooManyRequests(Duration),
    /// Anything unexpected, such as a database error: logged, then shown as a 500.
    Internal(anyhow::Error),
}

/// A shorthand used by every handler: `AppResult<Html<String>>` instead of the full `Result`.
pub type AppResult<T> = Result<T, AppError>;

#[derive(Template)]
#[template(path = "errors/error.html")]
struct ErrorPage<'a> {
    status: u16,
    title: &'a str,
    message: &'a str,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // `match` must cover every variant; adding a new kind of error without deciding how to show
        // it is a compile error.
        let (status, title, message) = match &self {
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                "Страницата не е пронајдена",
                "Адресата што ја отворивте не постои или е преместена.".to_string(),
            ),
            AppError::PageExpired => (
                StatusCode::from_u16(419).unwrap_or(StatusCode::FORBIDDEN),
                "Страницата истече",
                "Формуларот беше отворен предолго. Вратете се назад, освежете ја страницата и обидете се повторно.".to_string(),
            ),
            AppError::TooManyRequests(wait) => (
                StatusCode::TOO_MANY_REQUESTS,
                "Премногу обиди",
                format!("Обидете се повторно за {} секунди.", wait.as_secs().max(1)),
            ),
            AppError::Internal(error) => {
                // `{error:#}` prints the error with its whole chain of causes.
                tracing::error!("{error:#}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Настана грешка",
                    "Нешто тргна наопаку на нашата страна. Обидете се повторно подоцна.".to_string(),
                )
            }
        };

        let page = ErrorPage {
            status: status.as_u16(),
            title,
            message: &message,
        };
        let body = page.render().unwrap_or_else(|_| title.to_string());

        (status, Html(body)).into_response()
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
