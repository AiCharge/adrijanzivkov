//! What every request handler can reach: Laravel's service container, but explicit.

use sqlx::SqlitePool;

/// Shared by all requests. `Clone` is cheap here: `SqlitePool` is a handle to one shared
/// pool of connections, so cloning it copies a pointer, not the connections.
#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}
