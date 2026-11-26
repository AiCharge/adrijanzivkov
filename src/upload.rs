//! Reading forms that carry files (`multipart/form-data`), and checking that an upload is an image.

use axum::extract::Multipart;
use std::collections::HashMap;

/// The largest image accepted: 10 MB, as Laravel's `max:10240` rule.
pub const MAX_IMAGE_BYTES: usize = 10 * 1024 * 1024;

/// A multipart form read into memory: its text fields, and its file fields by name.
#[derive(Default)]
pub struct MultipartForm {
    fields: HashMap<String, String>,
    files: HashMap<String, Vec<u8>>,
}

impl MultipartForm {
    /// Reads every part of the request body. A file input left empty arrives as an empty part and
    /// is treated as "no file".
    pub async fn read(mut multipart: Multipart) -> anyhow::Result<Self> {
        let mut form = Self::default();

        // `while let` loops for as long as the pattern matches: here, until `next_field` returns `None`.
        while let Some(field) = multipart.next_field().await? {
            let Some(name) = field.name().map(str::to_string) else {
                continue;
            };
            let is_file = field.file_name().is_some();
            let bytes = field.bytes().await?;

            if is_file {
                if !bytes.is_empty() {
                    form.files.insert(name, bytes.to_vec());
                }
            } else {
                form.fields
                    .insert(name, String::from_utf8_lossy(&bytes).to_string());
            }
        }

        Ok(form)
    }

    /// A text field, trimmed, or `""` when it wasn't sent.
    pub fn text(&self, name: &str) -> String {
        self.fields
            .get(name)
            .map(|value| value.trim().to_string())
            .unwrap_or_default()
    }

    /// The bytes of an uploaded file. `remove` hands them over without copying (ownership moves out).
    pub fn take_file(&mut self, name: &str) -> Option<Vec<u8>> {
        self.files.remove(name)
    }
}

/// An image kind the site accepts, recognised from the file's first bytes, not its name: a renamed
/// `.php` file is still refused.
#[derive(Debug, PartialEq, Eq)]
pub enum ImageKind {
    Jpeg,
    Png,
    Gif,
    Webp,
}

impl ImageKind {
    pub fn detect(bytes: &[u8]) -> Option<Self> {
        // Slice patterns: `[0xFF, 0xD8, 0xFF, ..]` matches any slice starting with those three bytes.
        match bytes {
            [0xFF, 0xD8, 0xFF, ..] => Some(ImageKind::Jpeg),
            [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A, ..] => Some(ImageKind::Png),
            [b'G', b'I', b'F', b'8', ..] => Some(ImageKind::Gif),
            // WebP: "RIFF", four bytes of length, then "WEBP". A `match` guard (`if …`) checks the rest.
            _ if bytes.starts_with(b"RIFF") && bytes.get(8..12) == Some(b"WEBP".as_slice()) => {
                Some(ImageKind::Webp)
            }
            _ => None,
        }
    }

    pub fn extension(&self) -> &'static str {
        match self {
            ImageKind::Jpeg => "jpg",
            ImageKind::Png => "png",
            ImageKind::Gif => "gif",
            ImageKind::Webp => "webp",
        }
    }
}

/// Why an upload was refused, so each form can word the message for its own field.
#[derive(Debug, PartialEq, Eq)]
pub enum ImageProblem {
    NotAnImage,
    TooLarge,
}

/// Checks an upload: an image of an accepted kind, no larger than 10 MB.
pub fn check_image(bytes: &[u8]) -> Result<ImageKind, ImageProblem> {
    if bytes.len() > MAX_IMAGE_BYTES {
        return Err(ImageProblem::TooLarge);
    }
    ImageKind::detect(bytes).ok_or(ImageProblem::NotAnImage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognises_images_by_their_bytes() {
        assert_eq!(
            ImageKind::detect(&[0xFF, 0xD8, 0xFF, 0xE0]),
            Some(ImageKind::Jpeg)
        );
        assert_eq!(
            ImageKind::detect(b"\x89PNG\r\n\x1a\n...."),
            Some(ImageKind::Png)
        );
        assert_eq!(
            ImageKind::detect(b"RIFF\0\0\0\0WEBPVP8 "),
            Some(ImageKind::Webp)
        );
        assert_eq!(ImageKind::detect(b"<?php echo 'hi';"), None);
    }

    #[test]
    fn refuses_files_over_ten_megabytes() {
        let mut big = vec![0xFF, 0xD8, 0xFF];
        big.resize(MAX_IMAGE_BYTES + 1, 0);
        assert_eq!(check_image(&big), Err(ImageProblem::TooLarge));
    }
}
