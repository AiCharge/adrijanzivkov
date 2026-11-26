//! The database records, like `app/Models`. Unlike Eloquent, each struct lists its columns as
//! typed fields, and every query is written out in SQL.

mod donation_need;
mod news;
pub mod story_submission;

// `pub use` re-exports, so other code writes `models::News` instead of `models::news::News`.
pub use donation_need::DonationNeed;
pub use news::News;
pub use story_submission::StorySubmission;

/// The public address of an uploaded file, like Laravel's `Storage::url()` for the public disk.
/// Old seed data stores full URLs, which are kept as they are.
fn storage_url(path: &str) -> String {
    if path.starts_with("http://") || path.starts_with("https://") {
        path.to_string()
    } else {
        format!("/storage/{path}")
    }
}
