use askama::Template;
use axum::Form;
use axum::extract::State;
use axum::response::{Html, IntoResponse, Redirect, Response};
use std::time::Duration;

use super::render;
use crate::error::{AppError, AppResult};
use crate::models::StorySubmission;
use crate::session::Session;
use crate::site::{ClientIp, Layout, Site};
use crate::state::AppState;
use crate::validation::{Errors, is_accepted, trimmed};

/// At most 4 stories per minute from one address, like `throttle:4,1` on the Laravel route.
const MAX_PER_MINUTE: usize = 4;
const SUCCESS_MESSAGE: &str = "Вашата порака е успешно испратена. Ќе ве контактираме наскоро.";

#[derive(Template)]
#[template(path = "pages/story_submissions/create.html")]
struct CreatePage {
    layout: Layout,
    csrf_token: String,
    success: Option<String>,
    form: StoryForm,
    errors: Errors,
}

/// The submitted fields. `#[derive(Deserialize)]` lets axum's `Form` extractor fill it from the
/// `application/x-www-form-urlencoded` body; `#[serde(default)]` makes a missing field an empty
/// string instead of a rejected request, so validation can explain what's missing.
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct StoryForm {
    #[serde(rename = "_token")]
    token: String,
    name: String,
    phone: String,
    message: String,
    /// A ticked checkbox sends `agreed=1`; an unticked one sends nothing, hence `Option`.
    agreed: Option<String>,
}

impl StoryForm {
    /// The rules and messages of Laravel's `StoreStorySubmissionRequest`.
    fn validate(&self) -> Errors {
        let mut errors = Errors::default();

        errors.required("name", &self.name, "Името е задолжително.");
        errors.max_chars(
            "name",
            &self.name,
            255,
            "Името може да содржи најмногу 255 знаци.",
        );
        errors.required("phone", &self.phone, "Телефонскиот број е задолжителен.");
        errors.max_chars("phone", &self.phone, 50, "Телефонскиот број е предолг.");
        errors.required("message", &self.message, "Пораката е задолжителна.");
        errors.max_chars(
            "message",
            &self.message,
            5000,
            "Пораката може да содржи најмногу 5000 знаци.",
        );

        if !is_accepted(self.agreed.as_deref()) {
            errors.add(
                "agreed",
                "Мора да се согласите со Условите на користење и Политиката за приватност.",
            );
        }

        errors
    }

    fn is_agreed(&self) -> bool {
        is_accepted(self.agreed.as_deref())
    }
}

/// `GET /story-submissions/create`
pub async fn create(site: Site, mut session: Session) -> AppResult<impl IntoResponse> {
    let page = CreatePage {
        layout: Layout::new(&site, "Сподели ја својата приказна"),
        csrf_token: session.csrf_token(),
        success: session.take_flash(),
        form: StoryForm::default(),
        errors: Errors::default(),
    };

    // Returning `(session, html)` sends the page with the session's updated cookies.
    Ok((session, render(&page)?))
}

/// `POST /story-submissions`
///
/// On success it redirects back to the form with a thank-you message (Post/Redirect/Get, so a
/// refresh doesn't send the story twice). With errors it shows the form again, keeping what was typed.
pub async fn store(
    State(state): State<AppState>,
    site: Site,
    ClientIp(ip): ClientIp,
    mut session: Session,
    // `Form` must come last: it reads the request body, which can only be read once.
    Form(form): Form<StoryForm>,
) -> AppResult<Response> {
    session.verify_csrf(&form.token)?;

    state
        .rate_limiter
        .hit(
            &format!("story:{ip}"),
            MAX_PER_MINUTE,
            Duration::from_secs(60),
        )
        .map_err(AppError::TooManyRequests)?;

    let form = StoryForm {
        name: trimmed(&form.name),
        phone: trimmed(&form.phone),
        message: trimmed(&form.message),
        ..form
    };

    let errors = form.validate();

    if !errors.is_empty() {
        let page = CreatePage {
            layout: Layout::new(&site, "Сподели ја својата приказна"),
            csrf_token: session.csrf_token(),
            success: None,
            form,
            errors,
        };
        let html: Html<String> = render(&page)?;

        // 422 Unprocessable Content, as Laravel answers a failed validation.
        return Ok((axum::http::StatusCode::UNPROCESSABLE_ENTITY, session, html).into_response());
    }

    StorySubmission::create(&state.db, &form.name, &form.phone, &form.message).await?;

    session.flash(SUCCESS_MESSAGE);

    // 303 See Other tells the browser to follow up with a GET.
    Ok((session, Redirect::to("/story-submissions/create")).into_response())
}
