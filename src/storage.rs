//! Where uploaded images go, like Laravel's `Storage` facade with the `public` disk.
//!
//! Locally files are written under `storage/app/public` and served at `/storage/…`. On Laravel Cloud
//! the disk isn't kept between deploys, so files go to its S3-compatible object storage instead
//! (Chapter 4, `Storage::S3`). The rest of the app only calls `put` and `url`, so it doesn't care which.

use anyhow::Context;
use std::path::PathBuf;
use std::sync::OnceLock;

/// The public address uploads are served from: `/storage` locally, the bucket's URL on Laravel Cloud.
///
/// `OnceLock` is a value set once at startup and read anywhere afterwards, safely from any thread. It
/// plays the role of `config('filesystems.disks.public.url')`, so a model can build an image URL
/// without being handed the storage.
static PUBLIC_URL: OnceLock<String> = OnceLock::new();

/// The address of an uploaded file, like `Storage::url($path)`. Seed data that already holds a full
/// URL is returned unchanged.
pub fn url(path: &str) -> String {
    if path.starts_with("http://") || path.starts_with("https://") {
        return path.to_string();
    }

    let base = PUBLIC_URL.get().map(String::as_str).unwrap_or("/storage");
    format!(
        "{}/{}",
        base.trim_end_matches('/'),
        path.trim_start_matches('/')
    )
}

/// Where files are kept.
#[derive(Clone)]
pub enum Storage {
    /// A folder on this machine, served at `/storage`.
    Local { root: PathBuf },
}

impl Storage {
    /// Local storage in `storage/app/public`.
    pub fn local() -> Self {
        PUBLIC_URL.get_or_init(|| "/storage".to_string());
        Storage::Local {
            root: PathBuf::from("storage/app/public"),
        }
    }

    /// Local storage in another folder, e.g. a temporary one for tests.
    pub fn local_in(root: PathBuf) -> Self {
        PUBLIC_URL.get_or_init(|| "/storage".to_string());
        Storage::Local { root }
    }

    /// Saves a file under `directory` with a random name, like `$file->store('news')`, and returns its
    /// path (e.g. `news/k3J9….webp`), which is what the database keeps.
    pub async fn put(
        &self,
        directory: &str,
        extension: &str,
        bytes: Vec<u8>,
    ) -> anyhow::Result<String> {
        let path = format!("{directory}/{}.{extension}", random_name());

        match self {
            Storage::Local { root } => {
                let full_path = root.join(&path);
                if let Some(parent) = full_path.parent() {
                    tokio::fs::create_dir_all(parent).await?;
                }
                tokio::fs::write(&full_path, bytes)
                    .await
                    .with_context(|| format!("could not write {}", full_path.display()))?;
            }
        }

        Ok(path)
    }
}

/// 40 random letters and digits, the length of Laravel's `hashName()`.
fn random_name() -> String {
    use rand::RngExt;
    const CHARACTERS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::rng();
    (0..40)
        .map(|_| CHARACTERS[rng.random_range(0..CHARACTERS.len())] as char)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_urls_and_keeps_full_ones() {
        assert_eq!(url("news/a.webp"), "/storage/news/a.webp");
        assert_eq!(
            url("https://example.com/a.webp"),
            "https://example.com/a.webp"
        );
    }

    #[test]
    fn names_files_randomly() {
        let name = random_name();
        assert_eq!(name.len(), 40);
        assert_ne!(name, random_name());
    }
}
