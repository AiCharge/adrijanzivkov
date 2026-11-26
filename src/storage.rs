//! Where uploaded images go, like Laravel's `Storage` facade.
//!
//! Locally files are written under `storage/app/public` and served at `/storage/…`. On Laravel Cloud the
//! disk isn't kept between deploys, so with `FILESYSTEM_DISK=s3` files go to its S3-compatible object
//! storage instead, read from the same `AWS_*` variables Laravel used. The rest of the app only calls
//! `put` and `url`, so it doesn't care which.

use anyhow::Context;
use object_store::aws::{AmazonS3, AmazonS3Builder};
use object_store::path::Path as ObjectPath;
use object_store::{Attribute, Attributes, ObjectStore, PutOptions};
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};

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

/// Where files are kept. An `enum` with data in its variants: each kind of storage carries what it
/// needs, and `match` makes every method handle both.
#[derive(Clone)]
pub enum Storage {
    /// A folder on this machine, served at `/storage`.
    Local { root: PathBuf },
    /// An S3-compatible bucket. `Arc` shares one client between all requests.
    S3(Arc<AmazonS3>),
}

impl Storage {
    /// Picks the storage from the environment, like Laravel's `FILESYSTEM_DISK`.
    pub fn from_env() -> anyhow::Result<Self> {
        match std::env::var("FILESYSTEM_DISK").as_deref() {
            Ok("s3") => Self::s3_from_env(),
            _ => Ok(Self::local()),
        }
    }

    /// Local storage in `storage/app/public`.
    pub fn local() -> Self {
        Self::local_in(PathBuf::from("storage/app/public"))
    }

    /// Local storage in another folder, e.g. a temporary one for tests.
    pub fn local_in(root: PathBuf) -> Self {
        PUBLIC_URL.get_or_init(|| "/storage".to_string());
        Storage::Local { root }
    }

    /// The bucket from Laravel's variables: `AWS_BUCKET`, `AWS_DEFAULT_REGION`, `AWS_ACCESS_KEY_ID`,
    /// `AWS_SECRET_ACCESS_KEY`, and for S3-compatible services `AWS_ENDPOINT` and
    /// `AWS_USE_PATH_STYLE_ENDPOINT`. `AWS_URL` is the public address files are served from.
    fn s3_from_env() -> anyhow::Result<Self> {
        let variable = |name: &str| {
            std::env::var(name)
                .ok()
                .filter(|value| !value.is_empty())
                .with_context(|| format!("{name} must be set when FILESYSTEM_DISK=s3"))
        };

        let bucket = variable("AWS_BUCKET")?;
        let public_url = variable("AWS_URL")?;

        let mut builder = AmazonS3Builder::new()
            .with_bucket_name(&bucket)
            .with_region(variable("AWS_DEFAULT_REGION").unwrap_or_else(|_| "auto".to_string()))
            .with_access_key_id(variable("AWS_ACCESS_KEY_ID")?)
            .with_secret_access_key(variable("AWS_SECRET_ACCESS_KEY")?);

        if let Ok(endpoint) = variable("AWS_ENDPOINT") {
            builder = builder.with_endpoint(endpoint);
        }

        let path_style =
            std::env::var("AWS_USE_PATH_STYLE_ENDPOINT").is_ok_and(|value| value == "true");
        builder = builder.with_virtual_hosted_style_request(!path_style);

        let client = builder
            .build()
            .context("could not configure the S3 bucket")?;
        PUBLIC_URL.get_or_init(|| public_url);

        Ok(Storage::S3(Arc::new(client)))
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
            Storage::S3(client) => {
                // The content type lets browsers show the image straight from the bucket.
                let attributes =
                    Attributes::from_iter([(Attribute::ContentType, content_type(extension))]);
                let options = PutOptions {
                    attributes,
                    ..PutOptions::default()
                };
                client
                    .put_opts(&ObjectPath::from(path.as_str()), bytes.into(), options)
                    .await
                    .with_context(|| format!("could not upload {path} to the bucket"))?;
            }
        }

        Ok(path)
    }

    /// The local folder, for serving `/storage`; `None` when files are in a bucket.
    pub fn local_root(&self) -> Option<&PathBuf> {
        match self {
            Storage::Local { root } => Some(root),
            Storage::S3(_) => None,
        }
    }
}

fn content_type(extension: &str) -> &'static str {
    match extension {
        "jpg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "webp" => "image/webp",
        _ => "application/octet-stream",
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

    #[test]
    fn labels_uploads_with_their_type() {
        assert_eq!(content_type("webp"), "image/webp");
        assert_eq!(content_type("exe"), "application/octet-stream");
    }
}
