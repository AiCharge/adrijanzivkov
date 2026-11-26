use askama::Template;
use axum::extract::{Multipart, Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};

use super::{Admin, AdminContext, Section};
use crate::error::{AppError, AppResult};
use crate::models::News;
use crate::models::news::NewsInput;
use crate::pagination::{Page, PageQuery, Paginator};
use crate::routes::render;
use crate::state::AppState;
use crate::upload::{ImageProblem, MultipartForm, check_image};
use crate::validation::Errors;

#[derive(Template)]
#[template(path = "admin/pages/news/index.html")]
struct IndexPage {
    ctx: AdminContext,
    news: Paginator<News>,
    search: String,
}

/// The create and edit pages share one template; `news` is `Some` when editing.
#[derive(Template)]
#[template(path = "admin/pages/news/form.html")]
struct FormPage {
    ctx: AdminContext,
    news: Option<News>,
    values: Values,
    errors: Errors,
}

/// What the form shows in its fields: the record's values, or what was just typed.
#[derive(Default)]
struct Values {
    title: String,
    short_description: String,
    content: String,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct Filters {
    search: String,
    page: Option<String>,
}

/// `GET /admin/news?search=…`
pub async fn index(
    State(state): State<AppState>,
    mut admin: Admin,
    Query(filters): Query<Filters>,
) -> AppResult<Response> {
    let search = filters.search.trim().to_string();
    let page = Page::from_query(&PageQuery { page: filters.page }, 15);
    let news = News::paginate(&state.db, page, &search, "/admin/news").await?;

    let page = IndexPage {
        ctx: admin.context("Новости", Section::News),
        news,
        search,
    };
    Ok((admin.session, render(&page)?).into_response())
}

/// `GET /admin/news/create`
pub async fn create(mut admin: Admin) -> AppResult<Response> {
    let mut ctx = admin.context("Нова новост", Section::News);
    ctx.breadcrumb = Some("Нова новост".to_string());

    let page = FormPage {
        ctx,
        news: None,
        values: Values::default(),
        errors: Errors::default(),
    };
    Ok((admin.session, render(&page)?).into_response())
}

/// `GET /admin/news/{slug}/edit`
pub async fn edit(
    State(state): State<AppState>,
    mut admin: Admin,
    Path(slug): Path<String>,
) -> AppResult<Response> {
    let news = News::find_by_slug(&state.db, &slug)
        .await?
        .ok_or(AppError::NotFound)?;

    let mut ctx = admin.context(&news.title, Section::News);
    ctx.breadcrumb = Some(news.title.clone());

    let page = FormPage {
        ctx,
        values: Values {
            title: news.title.clone(),
            short_description: news.short_description.clone(),
            content: news.content.clone(),
        },
        news: Some(news),
        errors: Errors::default(),
    };
    Ok((admin.session, render(&page)?).into_response())
}

/// `POST /admin/news`
pub async fn store(
    State(state): State<AppState>,
    admin: Admin,
    multipart: Multipart,
) -> AppResult<Response> {
    save(state, admin, None, multipart).await
}

/// `POST /admin/news/{slug}`
pub async fn update(
    State(state): State<AppState>,
    admin: Admin,
    Path(slug): Path<String>,
    multipart: Multipart,
) -> AppResult<Response> {
    let news = News::find_by_slug(&state.db, &slug)
        .await?
        .ok_or(AppError::NotFound)?;
    save(state, admin, Some(news), multipart).await
}

/// Creating and editing share the checks, as Laravel's `UpdateNewsRequest` extended `StoreNewsRequest`.
/// The only difference: a new post must have an image, an edited one keeps its image unless replaced.
async fn save(
    state: AppState,
    mut admin: Admin,
    existing: Option<News>,
    multipart: Multipart,
) -> AppResult<Response> {
    let mut form = MultipartForm::read(multipart).await?;
    admin.session.verify_csrf(&form.text("_token"))?;

    let values = Values {
        title: form.text("title"),
        short_description: form.text("short_description"),
        content: form.text("content"),
    };
    let image = form.take_file("main_image");

    let mut errors = Errors::default();
    errors.required("title", &values.title, "Насловот е задолжителен.");
    errors.max_chars(
        "title",
        &values.title,
        255,
        "Насловот може да содржи најмногу 255 знаци.",
    );
    errors.required(
        "short_description",
        &values.short_description,
        "Краткиот опис е задолжителен.",
    );
    errors.max_chars(
        "short_description",
        &values.short_description,
        500,
        "Краткиот опис може да содржи најмногу 500 знаци.",
    );
    errors.required("content", &values.content, "Содржината е задолжителна.");

    let image_kind = match &image {
        Some(bytes) => match check_image(bytes) {
            Ok(kind) => Some(kind),
            Err(ImageProblem::NotAnImage) => {
                errors.add("main_image", "Датотеката мора да биде слика.");
                None
            }
            Err(ImageProblem::TooLarge) => {
                errors.add("main_image", "Сликата може да биде најмногу 10 MB.");
                None
            }
        },
        None if existing.is_none() => {
            errors.add("main_image", "Главната слика е задолжителна.");
            None
        }
        None => None,
    };

    if !errors.is_empty() {
        let title = existing.as_ref().map_or("Нова новост".to_string(), |news| {
            news.title.clone()
        });
        let mut ctx = admin.context(&title, Section::News);
        ctx.breadcrumb = Some(title);
        let page = FormPage {
            ctx,
            news: existing,
            values,
            errors,
        };
        return Ok((
            StatusCode::UNPROCESSABLE_ENTITY,
            admin.session,
            render(&page)?,
        )
            .into_response());
    }

    // Saved only after every check passed, so a refused form leaves no stray file behind.
    let image_path = match (image, image_kind) {
        (Some(bytes), Some(kind)) => {
            Some(state.storage.put("news", kind.extension(), bytes).await?)
        }
        _ => None,
    };

    let input = NewsInput {
        title: &values.title,
        short_description: &values.short_description,
        content: &values.content,
    };

    match existing {
        None => {
            // `expect` documents why this can't fail: a new post without an image was refused above.
            let image_path = image_path.expect("a new post always has an image here");
            News::create(&state.db, &input, &image_path).await?;
            admin.session.flash("Новоста е креирана.");
            Ok((admin.session, Redirect::to("/admin/news")).into_response())
        }
        Some(news) => {
            News::update(&state.db, news.id, &input, image_path.as_deref()).await?;
            admin.session.flash("Новоста е зачувана.");
            let target = format!("/admin/news/{}/edit", news.slug);
            Ok((admin.session, Redirect::to(&target)).into_response())
        }
    }
}
