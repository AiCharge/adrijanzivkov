//! The database records, like `app/Models`. Unlike Eloquent, each struct lists its columns as
//! typed fields, and every query is written out in SQL.

pub mod donation_need;
pub mod news;
pub mod story_submission;
mod user;

// `pub use` re-exports, so other code writes `models::News` instead of `models::news::News`.
pub use donation_need::DonationNeed;
pub use news::News;
pub use story_submission::StorySubmission;
pub use user::User;

/// The public address of an uploaded file (see `storage::url`).
fn storage_url(path: &str) -> String {
    crate::storage::url(path)
}
