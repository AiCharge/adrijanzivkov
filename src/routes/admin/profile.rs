use askama::Template;
use axum::Form;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};

use super::{Admin, AdminContext, Section};
use crate::error::AppResult;
use crate::models::User;
use crate::routes::render;
use crate::state::AppState;
use crate::validation::{Errors, is_email, is_strong_password};

#[derive(Template)]
#[template(path = "admin/pages/profile/edit.html")]
struct ProfilePage {
    ctx: AdminContext,
    name: String,
    email: String,
    errors: Errors,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ProfileForm {
    #[serde(rename = "_token")]
    token: String,
    name: String,
    email: String,
    current_password: String,
    password: String,
    password_confirmation: String,
}

/// `GET /admin/profile`
pub async fn edit(mut admin: Admin) -> AppResult<Response> {
    let page = ProfilePage {
        ctx: admin.context("Мој профил", Section::Profile),
        name: admin.user.name.clone(),
        email: admin.user.email.clone(),
        errors: Errors::default(),
    };

    Ok((admin.session, render(&page)?).into_response())
}

/// `POST /admin/profile`: name and email, and the password when a new one is typed.
pub async fn update(
    State(state): State<AppState>,
    mut admin: Admin,
    Form(form): Form<ProfileForm>,
) -> AppResult<Response> {
    admin.session.verify_csrf(&form.token)?;

    let name = form.name.trim().to_string();
    let email = form.email.trim().to_string();
    let changes_password = !form.password.is_empty();

    let mut errors = Errors::default();
    errors.required("name", &name, "Името е задолжително.");
    errors.max_chars(
        "name",
        &name,
        255,
        "Името може да содржи најмногу 255 знаци.",
    );
    errors.required("email", &email, "Е-поштата е задолжителна.");
    if !email.is_empty() && !is_email(&email) {
        errors.add("email", "Внесете валидна е-пошта.");
    }
    errors.max_chars(
        "email",
        &email,
        255,
        "Е-поштата може да содржи најмногу 255 знаци.",
    );
    if !errors.has("email") && User::email_taken(&state.db, &email, admin.user.id).await? {
        errors.add("email", "Оваа е-пошта веќе се користи.");
    }

    if changes_password {
        if !is_strong_password(&form.password) {
            errors.add(
                "password",
                "Лозинката мора да има најмалку 12 знаци, мали и големи букви, број и симбол.",
            );
        } else if form.password != form.password_confirmation {
            errors.add("password", "Лозинките не се совпаѓаат.");
        }

        if form.current_password.is_empty() {
            errors.add(
                "current_password",
                "Внесете ја тековната лозинка за да ја промените лозинката.",
            );
        } else if !admin
            .user
            .password_matches(&state.db, &form.current_password)
            .await?
        {
            errors.add("current_password", "Тековната лозинка не е точна.");
        }
    }

    if !errors.is_empty() {
        let page = ProfilePage {
            ctx: admin.context("Мој профил", Section::Profile),
            name,
            email,
            errors,
        };
        return Ok((
            StatusCode::UNPROCESSABLE_ENTITY,
            admin.session,
            render(&page)?,
        )
            .into_response());
    }

    let new_password = changes_password.then_some(form.password.as_str());
    User::update_profile(&state.db, admin.user.id, &name, &email, new_password).await?;

    admin.session.flash("Профилот е зачуван.");
    Ok((admin.session, Redirect::to("/admin/profile")).into_response())
}
