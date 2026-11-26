//! The admin: login and its limits, guarding the pages, and each section's main actions.

mod common;

use adrijanzivkov::models::User;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use common::TestClient;

const PASSWORD: &str = "Тајна-Лозинка-2026";

/// A client with one admin account and some content.
async fn client_with_admin() -> TestClient {
    let client = TestClient::new().await;
    User::create(&client.state.db, "Admin", "admin@example.com", PASSWORD)
        .await
        .unwrap();
    client
        .sql(
            "INSERT INTO news (title, slug, short_description, main_image, content, created_at, updated_at) \
             VALUES ('Стара вест', 'stara-vest', 'Опис', 'news/a.webp', '<p>Текст</p>', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
        )
        .await;
    client
        .sql(
            "INSERT INTO donation_needs (title, description, image, needed_amount, created_at, updated_at) \
             VALUES ('Помош за Ана', '<p>Опис</p>', 'donation-needs/a.webp', 1000, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
        )
        .await;
    client
        .sql(
            "INSERT INTO story_submissions (name, phone, message, status, created_at, updated_at) \
             VALUES ('Марко', '070', 'Ни треба помош.', 'new', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
        )
        .await;
    client
}

async fn log_in(client: &mut TestClient) {
    let token = client.csrf_token("/admin/login").await;
    let response = client
        .post_form(
            "/admin/login",
            &[
                ("_token", &token),
                ("email", "admin@example.com"),
                ("password", PASSWORD),
            ],
        )
        .await;
    assert_eq!(response.status, StatusCode::SEE_OTHER, "{}", response.body);
}

/// Sends a multipart form (with files), like an admin form with an image.
async fn post_multipart(
    client: &mut TestClient,
    uri: &str,
    fields: &[(&str, &str)],
    file: Option<(&str, &[u8])>,
) -> common::TestResponse {
    let boundary = "----adrijan-test-boundary";
    let mut body: Vec<u8> = Vec::new();

    for (name, value) in fields {
        body.extend(
            format!(
                "--{boundary}\r\nContent-Disposition: form-data; name=\"{name}\"\r\n\r\n{value}\r\n"
            )
            .as_bytes(),
        );
    }
    if let Some((name, bytes)) = file {
        body.extend(
            format!(
                "--{boundary}\r\nContent-Disposition: form-data; name=\"{name}\"; filename=\"upload.png\"\r\nContent-Type: image/png\r\n\r\n"
            )
            .as_bytes(),
        );
        body.extend(bytes);
        body.extend(b"\r\n");
    }
    body.extend(format!("--{boundary}--\r\n").as_bytes());

    let request = Request::post(uri)
        .header(
            header::CONTENT_TYPE,
            format!("multipart/form-data; boundary={boundary}"),
        )
        .body(Body::from(body))
        .unwrap();
    client.send(request).await
}

/// The first bytes of a PNG file: enough for the image check.
const PNG: &[u8] = b"\x89PNG\r\n\x1a\n0000IHDR";

#[tokio::test]
async fn guests_are_sent_to_the_login_page_and_back_after_it() {
    let mut client = client_with_admin().await;

    for uri in [
        "/admin/donation-needs",
        "/admin/news",
        "/admin/story-submissions",
        "/admin/profile",
    ] {
        let response = client.get(uri).await;
        assert_eq!(response.status, StatusCode::SEE_OTHER, "{uri}");
        assert_eq!(response.location.as_deref(), Some("/admin/login"));
    }

    // The last page asked for was the profile: after logging in, that's where the admin lands.
    let token = client.csrf_token("/admin/login").await;
    let response = client
        .post_form(
            "/admin/login",
            &[
                ("_token", &token),
                ("email", "admin@example.com"),
                ("password", PASSWORD),
            ],
        )
        .await;
    assert_eq!(response.location.as_deref(), Some("/admin/profile"));
}

#[tokio::test]
async fn a_wrong_password_is_refused_and_five_lock_the_account_out() {
    let mut client = client_with_admin().await;
    let token = client.csrf_token("/admin/login").await;
    let wrong = [
        ("_token", token.as_str()),
        ("email", "admin@example.com"),
        ("password", "wrong"),
    ];

    let response = client.post_form("/admin/login", &wrong).await;
    assert_eq!(response.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(response.body.contains("Погрешна е-пошта или лозинка."));
    assert!(
        response.body.contains(r#"value="admin@example.com""#),
        "the email stays filled in"
    );

    for _ in 0..4 {
        client.post_form("/admin/login", &wrong).await;
    }

    // Even the right password is refused while locked out.
    let response = client
        .post_form(
            "/admin/login",
            &[
                ("_token", &token),
                ("email", "admin@example.com"),
                ("password", PASSWORD),
            ],
        )
        .await;
    assert!(response.body.contains("Премногу обиди за најава."));
    assert_eq!(
        client.get("/admin/news").await.status,
        StatusCode::SEE_OTHER
    );
}

#[tokio::test]
async fn logging_out_ends_the_session() {
    let mut client = client_with_admin().await;
    log_in(&mut client).await;
    assert_eq!(client.get("/admin/news").await.status, StatusCode::OK);

    let token = client.csrf_token("/admin/profile").await;
    let response = client
        .post_form("/admin/logout", &[("_token", &token)])
        .await;
    assert_eq!(response.location.as_deref(), Some("/admin/login"));
    assert_eq!(
        client.get("/admin/news").await.status,
        StatusCode::SEE_OTHER
    );
}

#[tokio::test]
async fn every_admin_page_opens_when_logged_in() {
    let mut client = client_with_admin().await;
    log_in(&mut client).await;

    for uri in [
        "/admin/donation-needs",
        "/admin/donation-needs?archived=archived&search=ана",
        "/admin/donation-needs/create",
        "/admin/donation-needs/1/edit",
        "/admin/news",
        "/admin/news/create",
        "/admin/news/stara-vest/edit",
        "/admin/story-submissions",
        "/admin/story-submissions?status=new",
        "/admin/story-submissions/1/edit",
        "/admin/profile",
    ] {
        let response = client.get(uri).await;
        assert_eq!(response.status, StatusCode::OK, "{uri}");
    }

    assert_eq!(
        client.get("/admin/news/nema/edit").await.status,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        client.get("/admin/donation-needs/99/edit").await.status,
        StatusCode::NOT_FOUND
    );
}

#[tokio::test]
async fn creates_a_news_post_with_an_image_and_a_unique_slug() {
    let mut client = client_with_admin().await;
    log_in(&mut client).await;
    let token = client.csrf_token("/admin/news/create").await;

    let fields = [
        ("_token", token.as_str()),
        ("title", "Стара вест"),
        ("short_description", "Краток опис"),
        ("content", "<p>Содржина</p>"),
    ];
    let response = post_multipart(
        &mut client,
        "/admin/news",
        &fields,
        Some(("main_image", PNG)),
    )
    .await;

    assert_eq!(
        response.location.as_deref(),
        Some("/admin/news"),
        "{}",
        response.body
    );
    assert_eq!(
        client.count("SELECT COUNT(*) FROM news WHERE slug = 'stara-vest-2' AND main_image LIKE 'news/%.png'").await,
        1
    );
    assert!(
        client
            .get("/admin/news")
            .await
            .body
            .contains("Новоста е креирана.")
    );
}

#[tokio::test]
async fn refuses_a_news_post_without_an_image_or_with_a_fake_one() {
    let mut client = client_with_admin().await;
    log_in(&mut client).await;
    let token = client.csrf_token("/admin/news/create").await;
    let fields = [
        ("_token", token.as_str()),
        ("title", ""),
        ("short_description", "Опис"),
        ("content", "<p>Содржина</p>"),
    ];

    let response = post_multipart(&mut client, "/admin/news", &fields, None).await;
    assert_eq!(response.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(response.body.contains("Насловот е задолжителен."));
    assert!(response.body.contains("Главната слика е задолжителна."));

    let response = post_multipart(
        &mut client,
        "/admin/news",
        &fields,
        Some(("main_image", b"<?php echo 1;")),
    )
    .await;
    assert!(response.body.contains("Датотеката мора да биде слика."));
    assert_eq!(client.count("SELECT COUNT(*) FROM news").await, 1);
}

#[tokio::test]
async fn edits_a_news_post_and_keeps_its_slug_and_image() {
    let mut client = client_with_admin().await;
    log_in(&mut client).await;
    let token = client.csrf_token("/admin/news/stara-vest/edit").await;

    let fields = [
        ("_token", token.as_str()),
        ("title", "Нов наслов"),
        ("short_description", "Нов опис"),
        ("content", "<p>Нова содржина</p>"),
    ];
    let response = post_multipart(&mut client, "/admin/news/stara-vest", &fields, None).await;

    assert_eq!(
        response.location.as_deref(),
        Some("/admin/news/stara-vest/edit")
    );
    assert_eq!(
        client.count("SELECT COUNT(*) FROM news WHERE slug = 'stara-vest' AND title = 'Нов наслов' AND main_image = 'news/a.webp'").await,
        1
    );
}

#[tokio::test]
async fn creates_and_validates_donation_needs() {
    let mut client = client_with_admin().await;
    log_in(&mut client).await;
    let token = client.csrf_token("/admin/donation-needs/create").await;

    let mut fields = vec![
        ("_token", token.as_str()),
        ("title", "Операција"),
        ("description", "<p>Опис</p>"),
        ("sticky_content", ""),
        ("needed_amount", "10.5"),
    ];
    let response = post_multipart(
        &mut client,
        "/admin/donation-needs",
        &fields,
        Some(("image", PNG)),
    )
    .await;
    assert_eq!(response.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(
        response
            .body
            .contains("Потребниот износ мора да биде цел број.")
    );

    fields[4] = ("needed_amount", "50000");
    let response = post_multipart(
        &mut client,
        "/admin/donation-needs",
        &fields,
        Some(("image", PNG)),
    )
    .await;
    assert_eq!(response.location.as_deref(), Some("/admin/donation-needs"));
    assert_eq!(
        client.count("SELECT COUNT(*) FROM donation_needs WHERE title = 'Операција' AND needed_amount = 50000 AND sticky_content IS NULL").await,
        1
    );
}

#[tokio::test]
async fn archives_and_restores_a_donation_need() {
    let mut client = client_with_admin().await;
    log_in(&mut client).await;
    let token = client.csrf_token("/admin/donation-needs").await;

    let response = client
        .post_form(
            "/admin/donation-needs/1/archive",
            &[
                ("_token", &token),
                ("_back", "/admin/donation-needs?archived=active"),
            ],
        )
        .await;
    assert_eq!(
        response.location.as_deref(),
        Some("/admin/donation-needs?archived=active")
    );
    assert_eq!(
        client
            .count("SELECT COUNT(*) FROM donation_needs WHERE archived_at IS NOT NULL")
            .await,
        1
    );
    assert!(
        !client.get("/").await.body.contains("Помош за Ана"),
        "archived needs leave the site"
    );

    // An outside address in `_back` is ignored.
    let response = client
        .post_form(
            "/admin/donation-needs/1/unarchive",
            &[("_token", &token), ("_back", "https://example.com")],
        )
        .await;
    assert_eq!(response.location.as_deref(), Some("/admin/donation-needs"));
    assert_eq!(
        client
            .count("SELECT COUNT(*) FROM donation_needs WHERE archived_at IS NULL")
            .await,
        1
    );
}

#[tokio::test]
async fn updates_only_a_storys_status_and_notes() {
    let mut client = client_with_admin().await;
    log_in(&mut client).await;
    let token = client.csrf_token("/admin/story-submissions/1/edit").await;

    let response = client
        .post_form(
            "/admin/story-submissions/1",
            &[("_token", &token), ("status", "bogus"), ("notes", "")],
        )
        .await;
    assert!(response.body.contains("Избраниот статус не е валиден."));

    client
        .post_form(
            "/admin/story-submissions/1",
            &[
                ("_token", &token),
                ("status", "completed"),
                ("notes", "Контактирано."),
                ("name", "Променето"),
            ],
        )
        .await;
    assert_eq!(
        client.count("SELECT COUNT(*) FROM story_submissions WHERE status = 'completed' AND notes = 'Контактирано.' AND name = 'Марко'").await,
        1
    );
}

#[tokio::test]
async fn changes_the_password_only_with_the_current_one() {
    let mut client = client_with_admin().await;
    log_in(&mut client).await;
    let token = client.csrf_token("/admin/profile").await;
    let new_password = "Нова-Лозинка-2027";

    let response = client
        .post_form(
            "/admin/profile",
            &[
                ("_token", &token),
                ("name", "Admin"),
                ("email", "admin@example.com"),
                ("current_password", "wrong"),
                ("password", new_password),
                ("password_confirmation", new_password),
            ],
        )
        .await;
    assert!(response.body.contains("Тековната лозинка не е точна."));

    let response = client
        .post_form(
            "/admin/profile",
            &[
                ("_token", &token),
                ("name", "Нов Админ"),
                ("email", "admin@example.com"),
                ("current_password", PASSWORD),
                ("password", new_password),
                ("password_confirmation", new_password),
            ],
        )
        .await;
    assert_eq!(response.location.as_deref(), Some("/admin/profile"));

    let user = User::authenticate(&client.state.db, "admin@example.com", new_password)
        .await
        .unwrap();
    assert_eq!(user.map(|user| user.name).as_deref(), Some("Нов Админ"));
}

#[tokio::test]
async fn admin_forms_need_the_csrf_token() {
    let mut client = client_with_admin().await;
    log_in(&mut client).await;

    let response = client
        .post_form("/admin/donation-needs/1/archive", &[("_token", "forged")])
        .await;
    assert_eq!(response.status.as_u16(), 419);
    assert_eq!(
        client
            .count("SELECT COUNT(*) FROM donation_needs WHERE archived_at IS NULL")
            .await,
        1
    );
}

#[tokio::test]
async fn the_editor_uploads_images_and_returns_their_address() {
    let mut client = client_with_admin().await;
    log_in(&mut client).await;
    let token = client.csrf_token("/admin/news/create").await;

    let boundary = "----editor";
    let mut body = format!(
        "--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"a.png\"\r\nContent-Type: image/png\r\n\r\n"
    )
    .into_bytes();
    body.extend(PNG);
    body.extend(format!("\r\n--{boundary}--\r\n").as_bytes());

    let request = Request::post("/admin/rich-content-attachments")
        .header(
            header::CONTENT_TYPE,
            format!("multipart/form-data; boundary={boundary}"),
        )
        .header("x-csrf-token", token)
        .body(Body::from(body))
        .unwrap();
    let response = client.send(request).await;

    assert_eq!(response.status, StatusCode::CREATED);
    assert!(
        response.body.contains(r#""url":"/storage/rich-content/"#),
        "{}",
        response.body
    );
}
