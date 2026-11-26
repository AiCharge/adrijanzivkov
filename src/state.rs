//! What every request handler can reach: Laravel's service container, but explicit.

use axum_extra::extract::cookie::Key;

use crate::db::Db;
use crate::rate_limit::RateLimiter;
use crate::storage::Storage;

/// Shared by all requests. Every field is cheap to `Clone`: each is a handle (a pointer) to one
/// shared value, so cloning the state for a request copies a few pointers, not the data.
#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    /// Encrypts and signs the session cookies (see `session.rs`).
    pub cookie_key: Key,
    pub rate_limiter: RateLimiter,
    /// Where uploaded images are saved (see `storage.rs`).
    pub storage: Storage,
}
