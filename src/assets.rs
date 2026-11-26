//! The files in `public/` (CSS, scripts, fonts, images, favicons), built into the program itself.
//!
//! `#[derive(Embed)]` copies every file into the binary when it's compiled in release mode, so a
//! deployment is one file: nothing to upload next to it, and nothing that can go missing. In debug builds
//! it reads the files from disk instead, so a rebuilt stylesheet shows up without recompiling.

use axum::http::{HeaderMap, HeaderValue, StatusCode, Uri, header};
use axum::response::{IntoResponse, Response};
use rust_embed::Embed;

use crate::error::AppError;

#[derive(Embed)]
#[folder = "public/"]
// Left over from the Laravel setup on a local checkout; never part of the site.
#[exclude = "storage/*"]
#[exclude = "build/*"]
#[exclude = "hot"]
struct PublicFiles;

/// Serves a file from `public/`, or the 404 page. Used as the router's fallback.
pub async fn serve(uri: Uri, headers: HeaderMap) -> Response {
    let path = uri.path().trim_start_matches('/');

    let Some(file) = PublicFiles::get(path) else {
        return AppError::NotFound.into_response();
    };

    // An ETag is a fingerprint of the file. When the browser already has this version it sends it back,
    // and the answer is a tiny "304 Not Modified" instead of the whole file.
    let etag = format!("\"{}\"", hex(&file.metadata.sha256_hash()[..8]));
    if headers
        .get(header::IF_NONE_MATCH)
        .is_some_and(|value| value.as_bytes() == etag.as_bytes())
    {
        return StatusCode::NOT_MODIFIED.into_response();
    }

    let mut response = file.data.into_owned().into_response();
    let response_headers = response.headers_mut();
    if let Ok(content_type) = HeaderValue::from_str(file.metadata.mimetype()) {
        response_headers.insert(header::CONTENT_TYPE, content_type);
    }
    if let Ok(etag) = HeaderValue::from_str(&etag) {
        response_headers.insert(header::ETAG, etag);
    }
    // Pages link to CSS and scripts with `?v=` (see `url`), so those can be kept for a year; other
    // files for a day.
    let cache = if uri.query().is_some_and(|query| query.starts_with("v=")) {
        "public, max-age=31536000, immutable"
    } else {
        "public, max-age=86400"
    };
    response_headers.insert(header::CACHE_CONTROL, HeaderValue::from_static(cache));

    response
}

/// A file's address with its fingerprint, e.g. `/css/app.css?v=3f2a9c1b`, so a new deployment's
/// stylesheet is never mixed up with an old one in a browser's cache. Templates call it as
/// `{{ crate::assets::url("css/app.css") }}`.
pub fn url(path: &str) -> String {
    match PublicFiles::get(path) {
        Some(file) => format!("/{path}?v={}", hex(&file.metadata.sha256_hash()[..4])),
        None => format!("/{path}"),
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
