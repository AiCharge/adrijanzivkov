//! The admin panel at `/admin`: login, the three content sections and the profile page.
//!
//! Every page but the login asks for an `Admin` in its parameters. That extractor is the whole of the
//! authentication check: a guest never reaches the handler's body, they're sent to the login page.

mod attachments;
mod auth;
mod donation_needs;
mod news;
mod profile;
mod story_submissions;

use axum::Router;
use axum::extract::{DefaultBodyLimit, FromRequestParts};
use axum::http::request::Parts;
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::{get, post};

use crate::models::User;
use crate::session::Session;
use crate::state::AppState;
use crate::upload::MAX_IMAGE_BYTES;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/admin",
            get(|| async { Redirect::to("/admin/donation-needs") }),
        )
        .route("/admin/login", get(auth::show).post(auth::login))
        .route("/admin/logout", post(auth::logout))
        .route("/admin/profile", get(profile::edit).post(profile::update))
        .route(
            "/admin/donation-needs",
            get(donation_needs::index).post(donation_needs::store),
        )
        .route("/admin/donation-needs/create", get(donation_needs::create))
        .route("/admin/donation-needs/{id}", post(donation_needs::update))
        .route("/admin/donation-needs/{id}/edit", get(donation_needs::edit))
        .route(
            "/admin/donation-needs/{id}/archive",
            post(donation_needs::archive),
        )
        .route(
            "/admin/donation-needs/{id}/unarchive",
            post(donation_needs::unarchive),
        )
        .route("/admin/news", get(news::index).post(news::store))
        .route("/admin/news/create", get(news::create))
        .route("/admin/news/{slug}", post(news::update))
        .route("/admin/news/{slug}/edit", get(news::edit))
        .route("/admin/story-submissions", get(story_submissions::index))
        .route(
            "/admin/story-submissions/{id}",
            post(story_submissions::update),
        )
        .route(
            "/admin/story-submissions/{id}/edit",
            get(story_submissions::edit),
        )
        .route("/admin/rich-content-attachments", post(attachments::store))
        // axum refuses request bodies over 2 MB by default; image uploads may be up to 10 MB.
        .layer(DefaultBodyLimit::max(MAX_IMAGE_BYTES + 1024 * 1024))
}

/// A logged-in administrator, with their session.
///
/// As an extractor it checks the login cookie and loads the user. If either fails, the *rejection*
/// (the error type axum returns instead of calling the handler) is a redirect to the login page.
pub struct Admin {
    pub user: User,
    pub session: Session,
}

impl FromRequestParts<AppState> for Admin {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // `Session`'s rejection is `Infallible` (it can't fail), so `match` needs only the `Ok` arm.
        let Ok(mut session) = Session::from_request_parts(parts, state).await;

        let user = match session.user_id() {
            Some(id) => User::find(&state.db, id).await.ok().flatten(),
            None => None,
        };

        match user {
            Some(user) => Ok(Admin { user, session }),
            None => {
                // Remember where they were going, for after the login (only for pages, not forms).
                if parts.method == axum::http::Method::GET {
                    let path = parts
                        .uri
                        .path_and_query()
                        .map(|p| p.as_str())
                        .unwrap_or("/admin");
                    session.set_intended(path);
                }
                Err((session, Redirect::to("/admin/login")).into_response())
            }
        }
    }
}

/// Which part of the admin a page belongs to: highlights the sidebar link and names the breadcrumb.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Section {
    DonationNeeds,
    News,
    StorySubmissions,
    Profile,
}

impl Section {
    fn group(self) -> &'static str {
        match self {
            Section::DonationNeeds | Section::News => "Содржина",
            Section::StorySubmissions => "Заедница",
            Section::Profile => "Профил",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Section::DonationNeeds => "Донации",
            Section::News => "Новости",
            Section::StorySubmissions => "Поднесени приказни",
            Section::Profile => "Мој профил",
        }
    }

    fn href(self) -> &'static str {
        match self {
            Section::DonationNeeds => "/admin/donation-needs",
            Section::News => "/admin/news",
            Section::StorySubmissions => "/admin/story-submissions",
            Section::Profile => "/admin/profile",
        }
    }
}

/// What the admin layout (`templates/admin/layouts/app.html`) needs on every page.
pub struct AdminContext {
    pub title: String,
    pub section: Section,
    /// The record a page shows, as the last breadcrumb, e.g. a post's title.
    pub breadcrumb: Option<String>,
    pub user: User,
    pub csrf_token: String,
    /// A message from the previous action, shown as a toast.
    pub flash: Option<String>,
}

impl AdminContext {
    pub fn group(&self) -> &'static str {
        self.section.group()
    }

    pub fn section_label(&self) -> &'static str {
        self.section.label()
    }

    pub fn section_href(&self) -> &'static str {
        self.section.href()
    }

    pub fn is_section(&self, name: &str) -> bool {
        let current = match self.section {
            Section::DonationNeeds => "donation-needs",
            Section::News => "news",
            Section::StorySubmissions => "story-submissions",
            Section::Profile => "profile",
        };
        current == name
    }
}

impl Admin {
    /// The layout data for a page. Takes the flash message, so it shows once.
    pub fn context(&mut self, title: &str, section: Section) -> AdminContext {
        AdminContext {
            title: title.to_string(),
            section,
            breadcrumb: None,
            user: self.user.clone(),
            csrf_token: self.session.csrf_token(),
            flash: self.session.take_flash(),
        }
    }
}

/// A local admin path to go back to after an action, from a form's `_back` field. Anything else (an
/// outside address, an empty value) falls back to `default`, so a form can't send someone elsewhere.
pub fn back_to(back: &str, default: &str) -> String {
    if back.starts_with("/admin") && !back.starts_with("//") {
        back.to_string()
    } else {
        default.to_string()
    }
}

/// One filter tab above a list, e.g. "Активни 3".
pub struct Tab {
    pub label: &'static str,
    pub href: String,
    pub count: i64,
    pub active: bool,
}

/// The query string a filter link needs, e.g. `?search=x&archived=active`.
pub fn query_string(params: &[(&str, &str)]) -> String {
    let encoded: Vec<String> = params
        .iter()
        .filter(|(_, value)| !value.is_empty())
        .map(|(name, value)| format!("{name}={}", crate::pagination::url_encode(value)))
        .collect();

    if encoded.is_empty() {
        String::new()
    } else {
        format!("?{}", encoded.join("&"))
    }
}
