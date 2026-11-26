use askama::Template;
use axum::response::Html;

use super::render;
use crate::error::AppResult;
use crate::site::{Layout, Site};

#[derive(Template)]
#[template(path = "pages/story_submissions/create.html")]
struct CreatePage {
    layout: Layout,
}

/// `GET /story-submissions/create`. Sending the form (`POST /story-submissions`) comes in
/// stage 2, with validation, CSRF protection, rate limiting and saving to the database.
pub async fn create(site: Site) -> AppResult<Html<String>> {
    render(&CreatePage {
        layout: Layout::new(&site, "Сподели ја својата приказна"),
    })
}
