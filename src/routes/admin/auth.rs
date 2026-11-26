use askama::Template;
use axum::Form;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use std::time::Duration;

use crate::error::{AppError, AppResult};
use crate::models::User;
use crate::routes::render;
use crate::session::Session;
use crate::site::ClientIp;
use crate::state::AppState;
use crate::validation::{Errors, is_accepted, is_email};

/// Five wrong passwords per email and address lock that pair out for a minute; ten login tries a minute
/// per address in total, as Laravel's `LoginRequest` and the route's `throttle:10,1` did.
const MAX_ATTEMPTS: usize = 5;
const MAX_PER_MINUTE: usize = 10;
const MINUTE: Duration = Duration::from_secs(60);

#[derive(Template)]
#[template(path = "admin/pages/auth/login.html")]
struct LoginPage {
    csrf_token: String,
    email: String,
    remember: bool,
    errors: Errors,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct LoginForm {
    #[serde(rename = "_token")]
    token: String,
    email: String,
    password: String,
    remember: Option<String>,
}

/// `GET /admin/login`. Someone already logged in goes straight to the admin.
pub async fn show(State(state): State<AppState>, mut session: Session) -> AppResult<Response> {
    if let Some(id) = session.user_id()
        && User::find(&state.db, id).await?.is_some()
    {
        return Ok(Redirect::to("/admin").into_response());
    }

    let page = LoginPage {
        csrf_token: session.csrf_token(),
        email: String::new(),
        remember: false,
        errors: Errors::default(),
    };

    Ok((session, render(&page)?).into_response())
}

/// `POST /admin/login`
pub async fn login(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    mut session: Session,
    Form(form): Form<LoginForm>,
) -> AppResult<Response> {
    session.verify_csrf(&form.token)?;

    state
        .rate_limiter
        .hit(&format!("login-ip:{ip}"), MAX_PER_MINUTE, MINUTE)
        .map_err(AppError::TooManyRequests)?;

    let email = form.email.trim().to_string();
    let remember = is_accepted(form.remember.as_deref());

    let mut errors = Errors::default();
    errors.required("email", &email, "Е-поштата е задолжителна.");
    if !email.is_empty() && !is_email(&email) {
        errors.add("email", "Внесете валидна е-пошта.");
    }
    errors.required("password", &form.password, "Лозинката е задолжителна.");

    if errors.is_empty() {
        let lockout_key = format!("login:{}|{ip}", email.to_lowercase());

        if let Some(wait) = state
            .rate_limiter
            .too_many(&lockout_key, MAX_ATTEMPTS, MINUTE)
        {
            errors.add(
                "email",
                format!(
                    "Премногу обиди за најава. Обидете се повторно за {} секунди.",
                    wait.as_secs().max(1)
                ),
            );
        } else {
            match User::authenticate(&state.db, &email, &form.password).await? {
                Some(user) => {
                    state.rate_limiter.clear(&lockout_key);
                    session.log_in(user.id, remember);
                    let destination = session
                        .take_intended()
                        .unwrap_or_else(|| "/admin".to_string());
                    return Ok((session, Redirect::to(&destination)).into_response());
                }
                None => {
                    // `let _ =` ignores the result on purpose: the lockout is checked above.
                    let _ = state.rate_limiter.hit(&lockout_key, MAX_ATTEMPTS, MINUTE);
                    errors.add("email", "Погрешна е-пошта или лозинка.");
                }
            }
        }
    }

    // The password is never sent back to the page.
    let page = LoginPage {
        csrf_token: session.csrf_token(),
        email,
        remember,
        errors,
    };

    Ok((StatusCode::UNPROCESSABLE_ENTITY, session, render(&page)?).into_response())
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct TokenForm {
    #[serde(rename = "_token")]
    token: String,
}

/// `POST /admin/logout`
pub async fn logout(mut session: Session, Form(form): Form<TokenForm>) -> AppResult<Response> {
    session.verify_csrf(&form.token)?;
    session.log_out();
    Ok((session, Redirect::to("/admin/login")).into_response())
}
