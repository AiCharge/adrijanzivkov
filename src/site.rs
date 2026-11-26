//! What every page's `<head>` needs: its title, SEO description and share image, and the
//! site's own address. Replaces the Blade root view's `$meta` and Inertia's `<Head title>`.

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use chrono::Datelike;
use std::convert::Infallible;

const SITE_NAME: &str = "adrijanzivkov.org";
const DEFAULT_TITLE: &str = "Во вечен спомен на Адријан Живков";
const DEFAULT_DESCRIPTION: &str = "Фондацијата „Адријан Живков“ е посветена на поддршка на семејствата, донации за деца и хуманитарни активности во заедницата.";

/// The address this request came in on, e.g. `http://localhost:3000` and `/news?page=2`.
///
/// This is a custom *extractor*: by implementing `FromRequestParts`, a handler can simply list
/// `site: Site` as a parameter and axum builds it from the request. It's how axum does what
/// Laravel's `Request $request` injection does, but each extractor pulls out only one thing.
pub struct Site {
    base_url: String,
    path_and_query: String,
}

impl<S: Send + Sync> FromRequestParts<S> for Site {
    // `Infallible` means this extractor can never fail, so it needs no error response.
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let header = |name: &str| {
            parts
                .headers
                .get(name)
                .and_then(|value| value.to_str().ok())
        };

        // Behind Laravel Cloud's load balancer the original scheme arrives in `X-Forwarded-Proto`.
        let scheme = header("x-forwarded-proto").unwrap_or("http");
        let host = header("host").unwrap_or("localhost");

        let path_and_query = parts
            .uri
            .path_and_query()
            .map(|value| value.as_str().to_string())
            .unwrap_or_else(|| "/".to_string());

        Ok(Self {
            base_url: format!("{scheme}://{host}"),
            path_and_query,
        })
    }
}

impl Site {
    /// Turns a path such as `/storage/news/a.webp` into a full address, for share previews.
    pub fn absolute(&self, path: &str) -> String {
        if path.starts_with("http://") || path.starts_with("https://") {
            path.to_string()
        } else {
            format!("{}{path}", self.base_url)
        }
    }
}

/// The values the base template (`templates/layouts/base.html`) puts in `<head>`.
pub struct Layout {
    /// The browser tab title, e.g. `Новости - adrijanzivkov.org`.
    pub tab_title: String,
    /// The title for share previews, e.g. `Новости | adrijanzivkov.org`.
    pub share_title: String,
    pub description: String,
    pub image: String,
    pub url: String,
    pub year: i32,
}

impl Layout {
    /// A page with the default description and share image. `title` names the page in the tab.
    pub fn new(site: &Site, title: &str) -> Self {
        Self {
            tab_title: format!("{title} - {SITE_NAME}"),
            share_title: format!("{DEFAULT_TITLE} | {SITE_NAME}"),
            description: DEFAULT_DESCRIPTION.to_string(),
            image: site.absolute("/images/meta/default.jpg"),
            url: site.absolute(&site.path_and_query),
            year: chrono::Utc::now().year(),
        }
    }

    /// Sets the share title and description, as the controllers did with `withViewData(['meta' => …])`.
    ///
    /// Taking `mut self` and returning `Self` lets calls chain: `Layout::new(..).meta(..).image(..)`.
    pub fn meta(mut self, title: &str, description: &str) -> Self {
        self.share_title = format!("{title} | {SITE_NAME}");
        self.description = description.to_string();
        self
    }

    /// Sets the share image to a record's own picture.
    pub fn image(mut self, site: &Site, path: &str) -> Self {
        self.image = site.absolute(path);
        self
    }
}
