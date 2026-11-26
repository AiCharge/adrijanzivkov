use askama::Template;
use axum::response::Html;

use super::render;
use crate::error::AppResult;
use crate::site::{Layout, Site};

#[derive(Template)]
#[template(path = "pages/legal/terms.html")]
struct TermsPage {
    layout: Layout,
}

#[derive(Template)]
#[template(path = "pages/legal/privacy.html")]
struct PrivacyPage {
    layout: Layout,
}

/// `GET /terms-of-use`
pub async fn terms(site: Site) -> AppResult<Html<String>> {
    render(&TermsPage {
        layout: Layout::new(&site, "Услови на користење"),
    })
}

/// `GET /privacy-policy`
pub async fn privacy(site: Site) -> AppResult<Html<String>> {
    render(&PrivacyPage {
        layout: Layout::new(&site, "Политика за приватност"),
    })
}
