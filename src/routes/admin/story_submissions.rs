use askama::Template;
use axum::Form;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};

use super::{Admin, AdminContext, Section, Tab, query_string};
use crate::error::{AppError, AppResult};
use crate::models::StorySubmission;
use crate::models::story_submission::Status;
use crate::pagination::{Page, Paginator};
use crate::routes::render;
use crate::state::AppState;
use crate::validation::Errors;

#[derive(Template)]
#[template(path = "admin/pages/story_submissions/index.html")]
struct IndexPage {
    ctx: AdminContext,
    submissions: Paginator<StorySubmission>,
    tabs: Vec<Tab>,
    search: String,
    status: String,
}

#[derive(Template)]
#[template(path = "admin/pages/story_submissions/edit.html")]
struct EditPage {
    ctx: AdminContext,
    submission: StorySubmission,
    statuses: [Status; 4],
    status: String,
    notes: String,
    errors: Errors,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct Filters {
    search: String,
    status: String,
    page: Option<String>,
}

/// `GET /admin/story-submissions?search=…&status=…`
pub async fn index(
    State(state): State<AppState>,
    mut admin: Admin,
    Query(filters): Query<Filters>,
) -> AppResult<Response> {
    let search = filters.search.trim().to_string();
    let status = Status::parse(&filters.status);
    let page = Page::from_query(
        &crate::pagination::PageQuery {
            page: filters.page.clone(),
        },
        15,
    );

    let (submissions, counts) = tokio::try_join!(
        StorySubmission::paginate(&state.db, page, &search, status),
        StorySubmission::counts(&state.db),
    )?;

    let count_of = |value: &str| {
        counts
            .iter()
            .find(|(key, _)| key == value)
            .map_or(0, |(_, count)| *count)
    };

    // "All" first, then one tab per status, each keeping the search.
    let mut tabs = vec![Tab {
        label: "Сите",
        href: format!(
            "/admin/story-submissions{}",
            query_string(&[("search", &search)])
        ),
        count: count_of("all"),
        active: status.is_none(),
    }];
    tabs.extend(Status::ALL.into_iter().map(|option| Tab {
        label: option.label(),
        href: format!(
            "/admin/story-submissions{}",
            query_string(&[("search", &search), ("status", option.value())])
        ),
        count: count_of(option.value()),
        active: status == Some(option),
    }));

    let page = IndexPage {
        ctx: admin.context("Поднесени приказни", Section::StorySubmissions),
        submissions,
        tabs,
        search,
        status: status.map(Status::value).unwrap_or("").to_string(),
    };

    Ok((admin.session, render(&page)?).into_response())
}

/// `GET /admin/story-submissions/{id}/edit`
pub async fn edit(
    State(state): State<AppState>,
    mut admin: Admin,
    Path(id): Path<String>,
) -> AppResult<Response> {
    let submission = find(&state, &id).await?;
    let mut ctx = admin.context(&submission.name, Section::StorySubmissions);
    ctx.breadcrumb = Some(submission.name.clone());

    let page = EditPage {
        ctx,
        status: submission.status().value().to_string(),
        notes: submission.notes.clone().unwrap_or_default(),
        submission,
        statuses: Status::ALL,
        errors: Errors::default(),
    };

    Ok((admin.session, render(&page)?).into_response())
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct UpdateForm {
    #[serde(rename = "_token")]
    token: String,
    status: String,
    notes: String,
}

/// `POST /admin/story-submissions/{id}`: only the status and notes change.
pub async fn update(
    State(state): State<AppState>,
    mut admin: Admin,
    Path(id): Path<String>,
    Form(form): Form<UpdateForm>,
) -> AppResult<Response> {
    admin.session.verify_csrf(&form.token)?;
    let submission = find(&state, &id).await?;
    let notes = form.notes.trim().to_string();

    let mut errors = Errors::default();
    errors.required("status", &form.status, "Статусот е задолжителен.");
    let status = Status::parse(&form.status);
    if status.is_none() {
        errors.add("status", "Избраниот статус не е валиден.");
    }
    errors.max_chars(
        "notes",
        &notes,
        10_000,
        "Белешките може да содржат најмногу 10000 знаци.",
    );

    // `let … else` either binds the value or runs the `else` block, which must leave the function.
    let (true, Some(status)) = (errors.is_empty(), status) else {
        let mut ctx = admin.context(&submission.name, Section::StorySubmissions);
        ctx.breadcrumb = Some(submission.name.clone());
        let page = EditPage {
            ctx,
            submission,
            statuses: Status::ALL,
            status: form.status,
            notes,
            errors,
        };
        return Ok((
            StatusCode::UNPROCESSABLE_ENTITY,
            admin.session,
            render(&page)?,
        )
            .into_response());
    };

    let notes = (!notes.is_empty()).then_some(notes.as_str());
    StorySubmission::update(&state.db, submission.id, status, notes).await?;

    admin.session.flash("Приказната е зачувана.");
    Ok((
        admin.session,
        Redirect::to(&format!("/admin/story-submissions/{}/edit", submission.id)),
    )
        .into_response())
}

async fn find(state: &AppState, id: &str) -> AppResult<StorySubmission> {
    let id: i64 = id.parse().map_err(|_| AppError::NotFound)?;
    StorySubmission::find(&state.db, id)
        .await?
        .ok_or(AppError::NotFound)
}
