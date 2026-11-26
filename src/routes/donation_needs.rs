use askama::Template;
use axum::extract::{Path, Query, State};
use axum::response::Html;

use super::render;
use crate::error::{AppError, AppResult};
use crate::format;
use crate::models::DonationNeed;
use crate::pagination::{Page, PageQuery, Paginator};
use crate::site::{Layout, Site};
use crate::state::AppState;

#[derive(Template)]
#[template(path = "pages/donation_needs/index.html")]
struct DonationNeedsIndexPage {
    layout: Layout,
    donation_needs: Paginator<DonationNeed>,
}

#[derive(Template)]
#[template(path = "pages/donation_needs/show.html")]
struct DonationNeedShowPage {
    layout: Layout,
    donation_need: DonationNeed,
}

/// `GET /donation-needs?page=2`
pub async fn index(
    State(state): State<AppState>,
    site: Site,
    Query(query): Query<PageQuery>,
) -> AppResult<Html<String>> {
    let donation_needs =
        DonationNeed::paginate_active(&state.db, Page::from_query(&query, 9)).await?;

    let layout = Layout::new(&site, "Потреба за донации").meta(
        "Потреба за донации",
        "Погледнете ги тековните потреби за донации на Фондацијата „Адријан Живков\" и помогнете таму каде е најпотребно.",
    );

    render(&DonationNeedsIndexPage {
        layout,
        donation_needs,
    })
}

/// `GET /donation-needs/{id}`
pub async fn show(
    State(state): State<AppState>,
    site: Site,
    Path(id): Path<String>,
) -> AppResult<Html<String>> {
    // The id arrives as text. `/donation-needs/abc` should be a 404, as in Laravel, not a 400,
    // so it's parsed here instead of letting axum reject it.
    let id: i64 = id.parse().map_err(|_| AppError::NotFound)?;

    let donation_need = DonationNeed::find(&state.db, id)
        .await?
        .ok_or(AppError::NotFound)?;

    let description = format::excerpt(&format::strip_html(&donation_need.description), 160);
    let layout = Layout::new(&site, &donation_need.title)
        .meta(&donation_need.title, &description)
        .image(&site, &donation_need.image_url());

    render(&DonationNeedShowPage {
        layout,
        donation_need,
    })
}
