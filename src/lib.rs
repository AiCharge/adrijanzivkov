//! The website of the Adrijan Zhivkov Foundation.
//!
//! This is a *library crate*: `main.rs` is a thin program that starts it, and the integration tests
//! in `tests/` use the same `app()` to send requests without a real network.
//!
//! Reading order for learning: `main.rs` → `config.rs` → `state.rs` → `routes/mod.rs` → one handler
//! such as `routes/news.rs` → `models/news.rs` → `templates/pages/news/index.html`. LEARNING.md has
//! the full tour.

// `pub mod` makes a module visible outside the crate (to main.rs and the tests); plain `mod` keeps it
// private to the library.
pub mod config;
pub mod db;
mod error;
mod format;
mod models;
mod pagination;
pub mod rate_limit;
mod routes;
pub mod session;
mod site;
pub mod state;
mod validation;

pub use routes::router as app;
