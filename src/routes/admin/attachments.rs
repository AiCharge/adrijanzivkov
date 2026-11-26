use axum::Json;
use axum::extract::{Multipart, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use serde_json::json;

use super::Admin;
use crate::error::AppResult;
use crate::state::AppState;
use crate::upload::{ImageProblem, MultipartForm, check_image};

/// `POST /admin/rich-content-attachments`: an image pasted into the rich-text editor.
///
/// The editor sends it with `fetch`, so it answers in JSON (`{"url": …}`, or Laravel's error shape
/// `{"errors": {"file": […]}}`) instead of a page. The CSRF token comes in the `X-CSRF-Token` header.
pub async fn store(
    State(state): State<AppState>,
    admin: Admin,
    headers: HeaderMap,
    multipart: Multipart,
) -> AppResult<Response> {
    let token = headers
        .get("x-csrf-token")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");
    admin.session.verify_csrf(token)?;

    let mut form = MultipartForm::read(multipart).await?;

    let refuse = |message: &str| {
        Ok((
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({ "errors": { "file": [message] } })),
        )
            .into_response())
    };

    let Some(bytes) = form.take_file("file") else {
        return refuse("Изберете слика.");
    };

    let kind = match check_image(&bytes) {
        Ok(kind) => kind,
        Err(ImageProblem::NotAnImage) => return refuse("Датотеката мора да биде слика."),
        Err(ImageProblem::TooLarge) => return refuse("Сликата може да биде најмногу 10 MB."),
    };

    let path = state
        .storage
        .put("rich-content", kind.extension(), bytes)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(json!({ "url": crate::storage::url(&path) })),
    )
        .into_response())
}
