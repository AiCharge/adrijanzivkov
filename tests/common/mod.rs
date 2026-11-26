//! Shared helpers for the integration tests: a fresh app on an in-memory database, and a tiny
//! client that sends requests straight to it (no network) and keeps cookies like a browser.

#![allow(dead_code)]

use adrijanzivkov::rate_limit::RateLimiter;
use adrijanzivkov::state::AppState;
use adrijanzivkov::storage::Storage;
use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use axum_extra::extract::cookie::Key;
use http_body_util::BodyExt;
use std::collections::HashMap;
use tower::ServiceExt;

/// What a request returned, with the body read as text.
pub struct TestResponse {
    pub status: StatusCode,
    pub location: Option<String>,
    pub body: String,
}

/// A browser-like client: remembers cookies between requests.
pub struct TestClient {
    pub app: Router,
    pub state: AppState,
    cookies: HashMap<String, String>,
}

impl TestClient {
    /// A new app with an empty, migrated in-memory SQLite database.
    pub async fn new() -> Self {
        let db = adrijanzivkov::db::connect("sqlite::memory:")
            .await
            .expect("database");
        let state = AppState {
            db,
            cookie_key: Key::generate(),
            rate_limiter: RateLimiter::default(),
            storage: Storage::local_in(
                std::env::temp_dir().join(format!("adrijanzivkov-test-{}", std::process::id())),
            ),
        };

        Self {
            app: adrijanzivkov::app(state.clone()),
            state,
            cookies: HashMap::new(),
        }
    }

    pub async fn get(&mut self, uri: &str) -> TestResponse {
        let request = Request::get(uri).body(Body::empty()).unwrap();
        self.send(request).await
    }

    /// Sends a form, like a browser submitting `<form method="post">`.
    pub async fn post_form(&mut self, uri: &str, fields: &[(&str, &str)]) -> TestResponse {
        let body = fields
            .iter()
            .map(|(name, value)| format!("{}={}", encode(name), encode(value)))
            .collect::<Vec<_>>()
            .join("&");

        let request = Request::post(uri)
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(Body::from(body))
            .unwrap();
        self.send(request).await
    }

    pub async fn send(&mut self, mut request: Request<Body>) -> TestResponse {
        if !self.cookies.is_empty() {
            let cookie_header = self
                .cookies
                .iter()
                .map(|(name, value)| format!("{name}={value}"))
                .collect::<Vec<_>>()
                .join("; ");
            request
                .headers_mut()
                .insert(header::COOKIE, cookie_header.parse().unwrap());
        }

        // `oneshot` sends one request to the router as if it came over the network.
        let response = self.app.clone().oneshot(request).await.unwrap();

        for set_cookie in response.headers().get_all(header::SET_COOKIE) {
            let set_cookie = set_cookie.to_str().unwrap();
            let pair = set_cookie.split(';').next().unwrap();
            let (name, value) = pair.split_once('=').unwrap();
            if value.is_empty() || set_cookie.contains("Max-Age=0") {
                self.cookies.remove(name);
            } else {
                self.cookies.insert(name.to_string(), value.to_string());
            }
        }

        let status = response.status();
        let location = response
            .headers()
            .get(header::LOCATION)
            .map(|value| value.to_str().unwrap().to_string());
        let bytes = response.into_body().collect().await.unwrap().to_bytes();

        TestResponse {
            status,
            location,
            body: String::from_utf8_lossy(&bytes).to_string(),
        }
    }

    /// Opens a page and returns the CSRF token from its form.
    pub async fn csrf_token(&mut self, uri: &str) -> String {
        let page = self.get(uri).await;
        csrf_token_from(&page.body).expect("the page has a CSRF token")
    }

    pub async fn sql(&self, query: &'static str) {
        sqlx::query(query).execute(&self.state.db).await.unwrap();
    }

    pub async fn count(&self, query: &'static str) -> i64 {
        sqlx::query_scalar(query)
            .fetch_one(&self.state.db)
            .await
            .unwrap()
    }
}

pub fn csrf_token_from(html: &str) -> Option<String> {
    let start = html.find(r#"name="_token" value=""#)? + r#"name="_token" value=""#.len();
    let end = html[start..].find('"')?;
    Some(html[start..start + end].to_string())
}

fn encode(value: &str) -> String {
    value
        .bytes()
        .map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                (byte as char).to_string()
            }
            _ => format!("%{byte:02X}"),
        })
        .collect()
}
