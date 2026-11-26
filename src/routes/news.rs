use askama::Template;
use axum::extract::{Path, Query, State};
use axum::response::Html;

use super::render;
use crate::error::{AppError, AppResult};
use crate::models::News;
use crate::pagination::{Page, PageQuery, Paginator};
use crate::site::{Layout, Site};
use crate::state::AppState;

#[derive(Template)]
#[template(path = "pages/news/index.html")]
struct NewsIndexPage {
    layout: Layout,
    news: Paginator<News>,
}

#[derive(Template)]
#[template(path = "pages/news/show.html")]
struct NewsShowPage {
    layout: Layout,
    news: News,
}

/// `GET /news?page=2`
pub async fn index(
    State(state): State<AppState>,
    site: Site,
    Query(query): Query<PageQuery>,
) -> AppResult<Html<String>> {
    let news = News::paginate(&state.db, Page::from_query(&query, 12)).await?;

    let layout = Layout::new(&site, "Новости").meta(
        "Новости",
        "Следете ги најновите вести и активности на Фондацијата „Адријан Живков\".",
    );

    render(&NewsIndexPage { layout, news })
}

/// `GET /news/{slug}`
pub async fn show(
    State(state): State<AppState>,
    site: Site,
    Path(slug): Path<String>,
) -> AppResult<Html<String>> {
    // `ok_or(…)?` turns "no row" (`None`) into the 404 error and returns it; otherwise it
    // unwraps the `Some(news)`. Laravel's route-model binding does this step for you.
    let news = News::find_by_slug(&state.db, &slug)
        .await?
        .ok_or(AppError::NotFound)?;

    let layout = Layout::new(&site, &news.title)
        .meta(&news.title, &news.short_description)
        .image(&site, &news.image_url());

    render(&NewsShowPage { layout, news })
}
