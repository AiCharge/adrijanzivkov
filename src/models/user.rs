use std::sync::LazyLock;

use crate::db::Db;

/// Someone who can log in to the admin.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}

/// Bcrypt's work factor: each password check takes a deliberate few hundred milliseconds, so
/// guessing passwords from a stolen database is slow. Laravel uses the same cost.
const BCRYPT_COST: u32 = 12;

impl User {
    /// Up to two letters for the avatar, e.g. `Admin Живков` → `AЖ`.
    pub fn initials(&self) -> String {
        self.name
            .split_whitespace()
            .take(2)
            .filter_map(|part| part.chars().next())
            .flat_map(char::to_uppercase)
            .collect()
    }

    pub async fn find(db: &Db, id: i64) -> sqlx::Result<Option<Self>> {
        sqlx::query_as("SELECT id, name, email FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await
    }

    /// Checks an email and password. Returns the user only when both match.
    pub async fn authenticate(
        db: &Db,
        email: &str,
        password: &str,
    ) -> anyhow::Result<Option<Self>> {
        let row: Option<(i64, String, String, String)> = sqlx::query_as(
            "SELECT id, name, email, password FROM users WHERE LOWER(email) = LOWER($1)",
        )
        .bind(email)
        .fetch_optional(db)
        .await?;

        // When the email doesn't exist, a hash is still checked (`None` → the dummy hash), so the answer
        // takes as long as for a real account and its timing doesn't reveal which emails have accounts.
        let hash = row.as_ref().map(|(_, _, _, hash)| hash.clone());
        let matches = verify_password(password, hash).await?;

        Ok(match row {
            Some((id, name, email, _)) if matches => Some(User { id, name, email }),
            _ => None,
        })
    }

    /// Checks a password against this user's stored hash (for "current password" on the profile).
    pub async fn password_matches(&self, db: &Db, password: &str) -> anyhow::Result<bool> {
        let hash: String = sqlx::query_scalar("SELECT password FROM users WHERE id = $1")
            .bind(self.id)
            .fetch_one(db)
            .await?;
        verify_password(password, Some(hash)).await
    }

    /// Whether another user already has this email.
    pub async fn email_taken(db: &Db, email: &str, except_id: i64) -> sqlx::Result<bool> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM users WHERE LOWER(email) = LOWER($1) AND id <> $2",
        )
        .bind(email)
        .bind(except_id)
        .fetch_one(db)
        .await?;
        Ok(count > 0)
    }

    pub async fn update_profile(
        db: &Db,
        id: i64,
        name: &str,
        email: &str,
        new_password: Option<&str>,
    ) -> anyhow::Result<()> {
        match new_password {
            Some(password) => {
                let hash = hash_password(password).await?;
                sqlx::query(
                    "UPDATE users SET name = $1, email = $2, password = $3, updated_at = CURRENT_TIMESTAMP WHERE id = $4",
                )
                .bind(name)
                .bind(email)
                .bind(hash)
                .bind(id)
                .execute(db)
                .await?;
            }
            None => {
                sqlx::query("UPDATE users SET name = $1, email = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3")
                    .bind(name)
                    .bind(email)
                    .bind(id)
                    .execute(db)
                    .await?;
            }
        }
        Ok(())
    }

    /// Creates a user. Used by tests and the `create-admin` command.
    pub async fn create(db: &Db, name: &str, email: &str, password: &str) -> anyhow::Result<()> {
        let hash = hash_password(password).await?;
        sqlx::query(
            "INSERT INTO users (name, email, password, created_at, updated_at) \
             VALUES ($1, $2, $3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
        )
        .bind(name)
        .bind(email)
        .bind(hash)
        .execute(db)
        .await?;
        Ok(())
    }
}

/// A real bcrypt hash of a throwaway password, checked when an email has no account. `LazyLock`
/// computes it the first time it's needed, then keeps it for the life of the program.
static DUMMY_HASH: LazyLock<String> =
    LazyLock::new(|| bcrypt::hash("not-a-real-password", BCRYPT_COST).unwrap_or_default());

/// Bcrypt deliberately takes a lot of CPU. `spawn_blocking` runs it on a thread meant for blocking
/// work, so the async threads keep serving other requests meanwhile. `move` hands the closure its own
/// copies of the values, because it may outlive this function's borrowed ones.
async fn verify_password(password: &str, hash: Option<String>) -> anyhow::Result<bool> {
    let password = password.to_string();
    let result = tokio::task::spawn_blocking(move || {
        let hash = hash.unwrap_or_else(|| DUMMY_HASH.clone());
        bcrypt::verify(password, &hash)
    })
    .await?;
    // A malformed stored hash counts as "doesn't match" rather than an error page.
    Ok(result.unwrap_or(false))
}

/// Hashes a password in Laravel's `$2y$` format, so Laravel could still read it.
pub async fn hash_password(password: &str) -> anyhow::Result<String> {
    let password = password.to_string();
    let parts =
        tokio::task::spawn_blocking(move || bcrypt::hash_with_result(password, BCRYPT_COST))
            .await??;
    Ok(parts.format_for_version(bcrypt::Version::TwoY))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn hashes_in_laravel_format_and_verifies() {
        let hash = hash_password("Тајна-Лозинка-2026").await.unwrap();
        assert!(hash.starts_with("$2y$12$"));
        assert!(
            verify_password("Тајна-Лозинка-2026", Some(hash.clone()))
                .await
                .unwrap()
        );
        assert!(!verify_password("wrong", Some(hash)).await.unwrap());
        assert!(!verify_password("anything", None).await.unwrap());
    }

    #[test]
    fn makes_initials() {
        let user = User {
            id: 1,
            name: "admin живков".to_string(),
            email: String::new(),
        };
        assert_eq!(user.initials(), "AЖ");
    }
}
