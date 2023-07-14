use anyhow::{anyhow, Result};
use sqlx::{Pool, Postgres};

#[tracing::instrument]
pub async fn is_user_exists(db_client: &Pool<Postgres>, email: String) -> Result<bool> {
    let user_exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM \"user\" WHERE email = $1)",
        email
    )
    .fetch_one(db_client)
    .await?;

    match user_exists {
        Some(result) => Ok(result),
        None => Err(anyhow!("Internal service error.")),
    }
}
