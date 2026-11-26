//! The public "share your story" form: CSRF, validation, saving, the thank-you message, rate limiting.

mod common;

use axum::http::StatusCode;
use common::TestClient;

const FORM: &str = "/story-submissions/create";

fn valid_fields(token: &str) -> Vec<(&str, &str)> {
    vec![
        ("_token", token),
        ("name", "Ана Петрова"),
        ("phone", "+389 70 000 000"),
        ("message", "Ни треба помош."),
        ("agreed", "1"),
    ]
}

#[tokio::test]
async fn saves_a_story_and_shows_the_thank_you_message_once() {
    let mut client = TestClient::new().await;
    let token = client.csrf_token(FORM).await;

    let response = client
        .post_form("/story-submissions", &valid_fields(&token))
        .await;
    assert_eq!(response.status, StatusCode::SEE_OTHER);
    assert_eq!(response.location.as_deref(), Some(FORM));

    assert_eq!(
        client.count("SELECT COUNT(*) FROM story_submissions WHERE name = 'Ана Петрова' AND status = 'new'").await,
        1
    );

    let page = client.get(FORM).await;
    assert!(page.body.contains("Вашата порака е успешно испратена."));

    let page = client.get(FORM).await;
    assert!(
        !page.body.contains("Вашата порака е успешно испратена."),
        "the message shows only once"
    );
}

#[tokio::test]
async fn refuses_a_form_without_the_csrf_token() {
    let mut client = TestClient::new().await;
    client.get(FORM).await;

    let response = client
        .post_form("/story-submissions", &valid_fields("forged"))
        .await;

    assert_eq!(response.status.as_u16(), 419);
    assert_eq!(
        client.count("SELECT COUNT(*) FROM story_submissions").await,
        0
    );
}

#[tokio::test]
async fn shows_every_error_and_keeps_what_was_typed() {
    let mut client = TestClient::new().await;
    let token = client.csrf_token(FORM).await;

    let response = client
        .post_form(
            "/story-submissions",
            &[
                ("_token", &token),
                ("name", "Ана"),
                ("phone", "   "),
                ("message", ""),
            ],
        )
        .await;

    assert_eq!(response.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(response.body.contains("Телефонскиот број е задолжителен."));
    assert!(response.body.contains("Пораката е задолжителна."));
    assert!(
        response
            .body
            .contains("Мора да се согласите со Условите на користење")
    );
    assert!(!response.body.contains("Името е задолжително."));
    assert!(
        response.body.contains(r#"value="Ана""#),
        "the name stays filled in"
    );
    assert_eq!(
        client.count("SELECT COUNT(*) FROM story_submissions").await,
        0
    );
}

#[tokio::test]
async fn refuses_text_that_is_too_long() {
    let mut client = TestClient::new().await;
    let token = client.csrf_token(FORM).await;
    let long_phone = "1".repeat(51);

    let mut fields = valid_fields(&token);
    fields[2] = ("phone", &long_phone);
    let response = client.post_form("/story-submissions", &fields).await;

    assert!(response.body.contains("Телефонскиот број е предолг."));
}

#[tokio::test]
async fn limits_each_visitor_to_four_stories_a_minute() {
    let mut client = TestClient::new().await;
    let token = client.csrf_token(FORM).await;

    for _ in 0..4 {
        let response = client
            .post_form("/story-submissions", &valid_fields(&token))
            .await;
        assert_eq!(response.status, StatusCode::SEE_OTHER);
    }

    let response = client
        .post_form("/story-submissions", &valid_fields(&token))
        .await;
    assert_eq!(response.status, StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(
        client.count("SELECT COUNT(*) FROM story_submissions").await,
        4
    );
}
