mod config;
mod external;
mod logger;

use config::load_env_vars;
use external::db;

#[tokio::main]
async fn main() {
    // Load environment variables and initialize logger
    load_env_vars();
    logger::init();
    // Create a new database client
    let db_client = db::init().await;
    // Automatically run migrations upon each service start
    // TODO: this is not the best practice
    //       we should use a centralized repository solely for dealing with database migrations
    //       but as an experimental project in early stage we will stick with this approach first
    db::migrate(&db_client).await;
}
