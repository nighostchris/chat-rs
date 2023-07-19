use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::Date;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub verified: bool,
    pub name: String,
    pub avatar: String,
    pub created_at: Date,
    pub updated_at: Date,
}

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
pub struct UserVerification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub secret: String,
    pub created_at: Date,
    pub updated_at: Date,
}
