use dotenvy::var;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use tracing::info;

pub mod models;
pub mod user;
pub mod user_verification;

// Initialize database client connection
#[tracing::instrument]
pub async fn init() -> Pool<Postgres> {
    // Try to get the environment variable 'DATABASE' that stores the database connection url
    let db_conn_url = var("DATABASE");

    match db_conn_url {
        Ok(url) => {
            info!("initializing database client");
            let pool = PgPoolOptions::new()
                .max_connections(20)
                .connect(url.as_str())
                .await;
            match pool {
                Ok(client) => {
                    info!("database client initialized");
                    client
                }
                Err(e) => panic!("Cannot initiate database connection. {}", e),
            }
        }
        Err(_) => panic!("Database connection url is invalid."),
    }
}

// Perform database migration
#[tracing::instrument]
pub async fn migrate(pool: &Pool<Postgres>) {
    match sqlx::migrate!().run(pool).await {
        Ok(_) => info!("database migration success"),
        Err(e) => panic!("Database migration failed. {}", e),
    };
}
