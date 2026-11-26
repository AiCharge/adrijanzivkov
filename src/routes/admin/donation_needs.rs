use askama::Template;
use axum::Form;
use axum::extract::{Multipart, Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};

use super::{Admin, AdminContext, Section, Tab, back_to, query_string};
use crate::error::{AppError, AppResult};
use crate::models::DonationNeed;
use crate::models::donation_need::{ArchiveFilter, DonationNeedInput};
use crate::pagination::{Page, PageQuery, Paginator};
use crate::routes::render;
use crate::state::AppState;
use crate::upload::{ImageProblem, MultipartForm, check_image};
use crate::validation::{Errors, parse_amount};

#[derive(Template)]
#[template(path = "admin/pages/donation_needs/index.html")]
struct IndexPage {
    ctx: AdminContext,
    donation_needs: Paginator<DonationNeed>,
    tabs: Vec<Tab>,
    search: String,
    archived: String,
    /// The list's own address, so archiving from it comes back to the same filters and page.
    current_url: String,
}

#[derive(Template)]
#[template(path = "admin/pages/donation_needs/form.html")]
struct FormPage {
    ctx: AdminContext,
    donation_need: Option<DonationNeed>,
    values: Values,
    errors: Errors,
}

#[derive(Default)]
struct Values {
    title: String,
    description: String,
    sticky_content: String,
    needed_amount: String,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct Filters {
    search: String,
    archived: String,
    page: Option<String>,
}

/// `GET /admin/donation-needs?search=…&archived=active|archived`
pub async fn index(
    State(state): State<AppState>,
    mut admin: Admin,
    Query(filters): Query<Filters>,
) -> AppResult<Response> {
    let search = filters.search.trim().to_string();
    let filter = ArchiveFilter::parse(&filters.archived);
    let page = Page::from_query(
        &PageQuery {
            page: filters.page.clone(),
        },
        15,
    );

    let (donation_needs, (all, active, archived)) = tokio::try_join!(
        DonationNeed::paginate(&state.db, page, &search, filter, "/admin/donation-needs"),
        DonationNeed::counts(&state.db),
    )?;

    let tab = |label, value: ArchiveFilter, count| Tab {
        label,
        href: format!(
            "/admin/donation-needs{}",
            query_string(&[("search", &search), ("archived", value.value())])
        ),
        count,
        active: filter == value,
    };
    let tabs = vec![
        tab("Сите", ArchiveFilter::All, all),
        tab("Активни", ArchiveFilter::Active, active),
        tab("Архивирани", ArchiveFilter::Archived, archived),
    ];

    let current_url = format!(
        "/admin/donation-needs{}",
        query_string(&[
            ("search", &search),
            ("archived", filter.value()),
            ("page", filters.page.as_deref().unwrap_or("")),
        ])
    );

    let page = IndexPage {
        ctx: admin.context("Донации", Section::DonationNeeds),
        donation_needs,
        tabs,
        archived: filter.value().to_string(),
        search,
        current_url,
    };
    Ok((admin.session, render(&page)?).into_response())
}

/// `GET /admin/donation-needs/create`
pub async fn create(mut admin: Admin) -> AppResult<Response> {
    let mut ctx = admin.context("Нова донациска потреба", Section::DonationNeeds);
    ctx.breadcrumb = Some("Нова донациска потреба".to_string());

    let page = FormPage {
        ctx,
        donation_need: None,
        values: Values::default(),
        errors: Errors::default(),
    };
    Ok((admin.session, render(&page)?).into_response())
}

/// `GET /admin/donation-needs/{id}/edit`
pub async fn edit(
    State(state): State<AppState>,
    mut admin: Admin,
    Path(id): Path<String>,
) -> AppResult<Response> {
    let donation_need = find(&state, &id).await?;
    let mut ctx = admin.context(&donation_need.title, Section::DonationNeeds);
    ctx.breadcrumb = Some(donation_need.title.clone());

    let page = FormPage {
        ctx,
        values: Values {
            title: donation_need.title.clone(),
            description: donation_need.description.clone(),
            sticky_content: donation_need.sticky_content.clone().unwrap_or_default(),
            needed_amount: donation_need.needed_amount.to_string(),
        },
        donation_need: Some(donation_need),
        errors: Errors::default(),
    };
    Ok((admin.session, render(&page)?).into_response())
}

/// `POST /admin/donation-needs`
pub async fn store(
    State(state): State<AppState>,
    admin: Admin,
    multipart: Multipart,
) -> AppResult<Response> {
    save(state, admin, None, multipart).await
}

/// `POST /admin/donation-needs/{id}`
pub async fn update(
    State(state): State<AppState>,
    admin: Admin,
    Path(id): Path<String>,
    multipart: Multipart,
) -> AppResult<Response> {
    let donation_need = find(&state, &id).await?;
    save(state, admin, Some(donation_need), multipart).await
}

async fn save(
    state: AppState,
    mut admin: Admin,
    existing: Option<DonationNeed>,
    multipart: Multipart,
) -> AppResult<Response> {
    let mut form = MultipartForm::read(multipart).await?;
    admin.session.verify_csrf(&form.text("_token"))?;

    let values = Values {
        title: form.text("title"),
        description: form.text("description"),
        sticky_content: form.text("sticky_content"),
        needed_amount: form.text("needed_amount"),
    };
    let image = form.take_file("image");

    let mut errors = Errors::default();
    errors.required("title", &values.title, "Насловот е задолжителен.");
    errors.max_chars(
        "title",
        &values.title,
        255,
        "Насловот може да содржи најмногу 255 знаци.",
    );
    errors.required("description", &values.description, "Описот е задолжителен.");

    let amount = parse_amount(&values.needed_amount)
        .map_err(|message| errors.add("needed_amount", message))
        .ok();

    let image_kind = match &image {
        Some(bytes) => match check_image(bytes) {
            Ok(kind) => Some(kind),
            Err(ImageProblem::NotAnImage) => {
                errors.add("image", "Датотеката мора да биде слика.");
                None
            }
            Err(ImageProblem::TooLarge) => {
                errors.add("image", "Сликата може да биде најмногу 10 MB.");
                None
            }
        },
        None if existing.is_none() => {
            errors.add("image", "Сликата е задолжителна.");
            None
        }
        None => None,
    };

    let (true, Some(amount)) = (errors.is_empty(), amount) else {
        let title = existing.as_ref().map_or(
            "Нова донациска потреба".to_string(),
            |need| need.title.clone(),
        );
        let mut ctx = admin.context(&title, Section::DonationNeeds);
        ctx.breadcrumb = Some(title);
        let page = FormPage {
            ctx,
            donation_need: existing,
            values,
            errors,
        };
        return Ok((
            StatusCode::UNPROCESSABLE_ENTITY,
            admin.session,
            render(&page)?,
        )
            .into_response());
    };

    let image_path = match (image, image_kind) {
        (Some(bytes), Some(kind)) => Some(
            state
                .storage
                .put("donation-needs", kind.extension(), bytes)
                .await?,
        ),
        _ => None,
    };

    let input = DonationNeedInput {
        title: &values.title,
        description: &values.description,
        // An empty editor is stored as NULL, as Laravel's ConvertEmptyStringsToNull did.
        sticky_content: (!values.sticky_content.is_empty())
            .then_some(values.sticky_content.as_str()),
        needed_amount: amount,
    };

    match existing {
        None => {
            let image_path = image_path.expect("a new donation need always has an image here");
            DonationNeed::create(&state.db, &input, &image_path).await?;
            admin.session.flash("Донациската потреба е креирана.");
            Ok((admin.session, Redirect::to("/admin/donation-needs")).into_response())
        }
        Some(need) => {
            DonationNeed::update(&state.db, need.id, &input, image_path.as_deref()).await?;
            admin.session.flash("Донациската потреба е зачувана.");
            let target = format!("/admin/donation-needs/{}/edit", need.id);
            Ok((admin.session, Redirect::to(&target)).into_response())
        }
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ArchiveForm {
    #[serde(rename = "_token")]
    token: String,
    #[serde(rename = "_back")]
    back: String,
}

/// `POST /admin/donation-needs/{id}/archive`: hides the need from the site.
pub async fn archive(
    State(state): State<AppState>,
    admin: Admin,
    Path(id): Path<String>,
    Form(form): Form<ArchiveForm>,
) -> AppResult<Response> {
    set_archived(state, admin, id, form, true).await
}

/// `POST /admin/donation-needs/{id}/unarchive`: shows it on the site again.
pub async fn unarchive(
    State(state): State<AppState>,
    admin: Admin,
    Path(id): Path<String>,
    Form(form): Form<ArchiveForm>,
) -> AppResult<Response> {
    set_archived(state, admin, id, form, false).await
}

async fn set_archived(
    state: AppState,
    mut admin: Admin,
    id: String,
    form: ArchiveForm,
    archived: bool,
) -> AppResult<Response> {
    admin.session.verify_csrf(&form.token)?;
    let need = find(&state, &id).await?;

    DonationNeed::set_archived(&state.db, need.id, archived).await?;

    admin.session.flash(if archived {
        "Донациската потреба е архивирана."
    } else {
        "Донациската потреба е вратена."
    });

    // Back to where the button was: the list (with its filters) or the edit page.
    let target = back_to(&form.back, "/admin/donation-needs");
    Ok((admin.session, Redirect::to(&target)).into_response())
}

async fn find(state: &AppState, id: &str) -> AppResult<DonationNeed> {
    let id: i64 = id.parse().map_err(|_| AppError::NotFound)?;
    DonationNeed::find(&state.db, id)
        .await?
        .ok_or(AppError::NotFound)
}
