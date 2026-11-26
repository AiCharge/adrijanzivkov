//! The public pages: each one opens, shows the database's content, and answers 404 when it should.

mod common;

use axum::http::StatusCode;
use common::TestClient;

async fn client_with_content() -> TestClient {
    let client = TestClient::new().await;
    client
        .sql(
            "INSERT INTO news (title, slug, short_description, main_image, content, created_at, updated_at) \
             VALUES ('Хуманитарна акција', 'humanitarna-akcija', 'Краток опис', 'news/a.webp', '<p>Содржина</p>', '2026-11-26 10:00:00', '2026-11-26 10:00:00')",
        )
        .await;
    client
        .sql(
            "INSERT INTO donation_needs (title, description, image, needed_amount, created_at, updated_at) \
             VALUES ('Лекување за Ана', '<p>Опис</p>', 'donation-needs/a.webp', 120000, '2026-11-26 10:00:00', '2026-11-26 10:00:00')",
        )
        .await;
    client
        .sql(
            "INSERT INTO donation_needs (title, description, image, needed_amount, archived_at, created_at, updated_at) \
             VALUES ('Архивирана потреба', '<p>Опис</p>', 'donation-needs/b.webp', 5000, '2026-11-26 11:00:00', '2026-11-26 10:00:00', '2026-11-26 10:00:00')",
        )
        .await;
    client
}

#[tokio::test]
async fn every_public_page_opens() {
    let mut client = client_with_content().await;

    for uri in [
        "/",
        "/about",
        "/news",
        "/news/humanitarna-akcija",
        "/donation-needs",
        "/donation-needs/1",
        "/terms-of-use",
        "/privacy-policy",
        "/story-submissions/create",
    ] {
        assert_eq!(client.get(uri).await.status, StatusCode::OK, "{uri}");
    }
}

#[tokio::test]
async fn the_home_page_shows_news_and_only_active_donation_needs() {
    let mut client = client_with_content().await;
    let page = client.get("/").await;

    assert!(page.body.contains("Хуманитарна акција"));
    assert!(page.body.contains("26 ноември 2026"));
    assert!(page.body.contains("Лекување за Ана"));
    assert!(page.body.contains("120.000 МКД"));
    assert!(!page.body.contains("Архивирана потреба"));
}

#[tokio::test]
async fn a_news_post_shows_its_html_content_and_share_tags() {
    let mut client = client_with_content().await;
    let page = client.get("/news/humanitarna-akcija").await;

    assert!(
        page.body.contains("<p>Содржина</p>"),
        "the admin's HTML is printed as HTML"
    );
    assert!(page.body.contains(
        r#"<meta property="og:title" content="Хуманитарна акција | adrijanzivkov.org">"#
    ));
    assert!(page.body.contains("/storage/news/a.webp"));
}

#[tokio::test]
async fn missing_pages_are_404() {
    let mut client = client_with_content().await;

    for uri in [
        "/news/does-not-exist",
        "/donation-needs/999",
        "/donation-needs/abc",
        "/nope",
    ] {
        let page = client.get(uri).await;
        assert_eq!(page.status, StatusCode::NOT_FOUND, "{uri}");
        assert!(page.body.contains("Страницата не е пронајдена"));
    }
}

#[tokio::test]
async fn a_bad_page_number_shows_the_first_page() {
    let mut client = client_with_content().await;
    let page = client.get("/news?page=abc").await;

    assert_eq!(page.status, StatusCode::OK);
    assert!(page.body.contains("Хуманитарна акција"));
}
