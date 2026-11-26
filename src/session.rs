//! The visitor's session, kept in encrypted cookies: the CSRF token, one-time "flash" messages and
//! who is logged in to the admin.
//!
//! Laravel stores sessions in a table and puts only an id in the cookie. Here the values themselves
//! live in the cookies, encrypted and signed with `APP_KEY` (axum-extra's `PrivateCookieJar`), so a
//! visitor can neither read nor change them, and the server stores nothing.

use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum::response::{IntoResponseParts, ResponseParts};
use axum_extra::extract::PrivateCookieJar;
use axum_extra::extract::cookie::{Cookie, Key, SameSite};
use base64::Engine;
use std::convert::Infallible;

use crate::error::AppError;
use crate::state::AppState;

const CSRF_COOKIE: &str = "csrf";
const FLASH_COOKIE: &str = "flash";
const USER_COOKIE: &str = "user";
const INTENDED_COOKIE: &str = "intended";

/// How long a login lasts. "Remember me" keeps it for 30 days, otherwise until the browser closes
/// or 12 hours pass, whichever comes first.
const SESSION_HOURS: i64 = 12;
const REMEMBER_DAYS: i64 = 30;

/// A request's session. As an extractor it reads the cookies; returned from a handler (it implements
/// `IntoResponseParts`) it writes back whatever changed.
pub struct Session {
    jar: PrivateCookieJar,
    secure: bool,
}

impl FromRequestParts<AppState> for Session {
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let jar = PrivateCookieJar::from_request_parts(parts, state).await?;
        // Mark cookies `Secure` (HTTPS only) whenever the site is served over HTTPS.
        let secure = parts
            .headers
            .get("x-forwarded-proto")
            .is_some_and(|value| value == "https");

        Ok(Self { jar, secure })
    }
}

/// Lets a handler return `(session, page)`: axum adds the session's `Set-Cookie` headers.
impl IntoResponseParts for Session {
    type Error = Infallible;

    fn into_response_parts(self, parts: ResponseParts) -> Result<ResponseParts, Self::Error> {
        self.jar.into_response_parts(parts)
    }
}

/// Tells `PrivateCookieJar` where to find the encryption key: in `AppState`. `FromRef` is how axum
/// extractors pull one piece out of the shared state.
impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.cookie_key.clone()
    }
}

impl Session {
    fn cookie(&self, name: &'static str, value: String) -> Cookie<'static> {
        Cookie::build((name, value))
            .path("/")
            .http_only(true)
            .same_site(SameSite::Lax)
            .secure(self.secure)
            .build()
    }

    /// `&mut self`: this method changes the session, so it needs exclusive access to it.
    fn set(&mut self, cookie: Cookie<'static>) {
        // `PrivateCookieJar::add` takes the jar by value and returns a new one, so we swap in a clone.
        self.jar = self.jar.clone().add(cookie);
    }

    fn remove(&mut self, name: &'static str) {
        self.jar = self.jar.clone().remove(Cookie::build(name).path("/"));
    }

    fn value(&self, name: &str) -> Option<String> {
        self.jar.get(name).map(|cookie| cookie.value().to_string())
    }

    // --- CSRF protection --------------------------------------------------------------------------

    /// The token every form must send back as `_token`, created on first use. Another site can make a
    /// visitor's browser submit a form here, but it can't read this token, so the forged form fails.
    pub fn csrf_token(&mut self) -> String {
        if let Some(token) = self.value(CSRF_COOKIE) {
            return token;
        }

        let token = random_token();
        let cookie = self.cookie(CSRF_COOKIE, token.clone());
        self.set(cookie);
        token
    }

    /// Checks a submitted token. A missing or wrong one is a 419, as in Laravel.
    pub fn verify_csrf(&self, submitted: &str) -> Result<(), AppError> {
        match self.value(CSRF_COOKIE) {
            Some(expected) if constant_time_eq(expected.as_bytes(), submitted.as_bytes()) => Ok(()),
            _ => Err(AppError::PageExpired),
        }
    }

    // --- Flash messages ---------------------------------------------------------------------------

    /// A message for the next page only, like `redirect()->with('success', …)`.
    pub fn flash(&mut self, message: &str) {
        let cookie = self.cookie(FLASH_COOKIE, message.to_string());
        self.set(cookie);
    }

    /// Reads the flash message and removes it, so it shows once.
    pub fn take_flash(&mut self) -> Option<String> {
        let message = self.value(FLASH_COOKIE)?;
        self.remove(FLASH_COOKIE);
        Some(message)
    }

    // --- Admin login ------------------------------------------------------------------------------

    /// Logs a user in. The cookie holds the user's id and when the login expires.
    pub fn log_in(&mut self, user_id: i64, remember: bool) {
        let now = chrono::Utc::now();
        let expires_at = if remember {
            now + chrono::Duration::days(REMEMBER_DAYS)
        } else {
            now + chrono::Duration::hours(SESSION_HOURS)
        };

        let mut cookie = self.cookie(USER_COOKIE, format!("{user_id}:{}", expires_at.timestamp()));
        if remember {
            cookie.set_max_age(time_duration_days(REMEMBER_DAYS));
        }
        self.set(cookie);

        // A new login gets a new CSRF token, like Laravel's `session()->regenerate()`.
        self.remove(CSRF_COOKIE);
    }

    /// The logged-in user's id, if the login hasn't expired.
    pub fn user_id(&self) -> Option<i64> {
        let value = self.value(USER_COOKIE)?;
        let (id, expires_at) = value.split_once(':')?;
        let expires_at: i64 = expires_at.parse().ok()?;

        if expires_at < chrono::Utc::now().timestamp() {
            return None;
        }

        id.parse().ok()
    }

    pub fn log_out(&mut self) {
        self.remove(USER_COOKIE);
        self.remove(CSRF_COOKIE);
    }

    /// Remembers the admin page a guest tried to open, to go there after logging in.
    pub fn set_intended(&mut self, path: &str) {
        let cookie = self.cookie(INTENDED_COOKIE, path.to_string());
        self.set(cookie);
    }

    /// The page to go to after logging in. Only local paths are accepted, so a link can't send
    /// someone to another site after they log in.
    pub fn take_intended(&mut self) -> Option<String> {
        let path = self.value(INTENDED_COOKIE)?;
        self.remove(INTENDED_COOKIE);
        (path.starts_with('/') && !path.starts_with("//")).then_some(path)
    }
}

/// The cookie crate measures time with the `time` crate, re-exported by `cookie`.
fn time_duration_days(days: i64) -> time::Duration {
    time::Duration::days(days)
}

/// 32 random bytes as URL-safe text.
fn random_token() -> String {
    let mut bytes = [0u8; 32];
    rand::fill(&mut bytes);
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

/// Compares two secrets in the same amount of time whatever they contain, so an attacker can't
/// learn a token one character at a time by timing the responses.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b).fold(0, |diff, (x, y)| diff | (x ^ y)) == 0
}

/// Builds the cookie encryption key from Laravel's `APP_KEY` (`base64:…`), or any long secret.
pub fn key_from_app_key(app_key: &str) -> anyhow::Result<Key> {
    let bytes = match app_key.strip_prefix("base64:") {
        Some(encoded) => base64::engine::general_purpose::STANDARD.decode(encoded)?,
        None => app_key.as_bytes().to_vec(),
    };

    anyhow::ensure!(bytes.len() >= 32, "APP_KEY must be at least 32 bytes long");
    Ok(Key::derive_from(&bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compares_tokens() {
        assert!(constant_time_eq(b"abc", b"abc"));
        assert!(!constant_time_eq(b"abc", b"abd"));
        assert!(!constant_time_eq(b"abc", b"ab"));
    }

    #[test]
    fn reads_laravel_style_keys() {
        assert!(key_from_app_key("base64:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=").is_ok());
        assert!(key_from_app_key("too short").is_err());
    }
}
