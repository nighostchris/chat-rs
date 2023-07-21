pub mod handlers;

use axum::routing::{get, post};
use axum::{Router, Server};
use dotenvy::var;
use handlers::health_check_handler;
use sqlx::{Pool, Postgres};
use std::net::SocketAddr;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;

#[derive(Debug)]
pub struct ServerState {
    db: Pool<Postgres>,
}

// Initialize an axum web server instance
#[tracing::instrument]
pub async fn init(db_client: Pool<Postgres>) {
    let server_state = Arc::new(ServerState { db: db_client });
    // https://stackoverflow.com/questions/74302133/how-to-log-and-filter-requests-with-axum-tokio
    let service = ServiceBuilder::new().layer(TraceLayer::new_for_http());
    // Define the routes for web server
    let user_routes = Router::new()
        .route("/register", post(handlers::user::register_handler))
        .route("/activate", get(handlers::user::activate_handler));
    let api_version_one_routes = Router::new().nest("/user", user_routes);
    let server = Router::new()
        .route("/", get(health_check_handler))
        .nest("/api/v1", api_version_one_routes)
        .layer(service)
        .with_state(server_state)
        .into_make_service();

    let web_server_host = var("WEB_SERVER_HOST");
    let web_server_port = var("WEB_SERVER_PORT");

    match web_server_host {
        Ok(host) => match web_server_port {
            Ok(port) => {
                info!(
                    "{}",
                    format!("web server is listening at {}:{}", host, port)
                );
                Server::bind(&format!("{}:{}", host, port).parse::<SocketAddr>().unwrap())
                    .serve(server)
                    .await
                    .unwrap();
            }
            Err(e) => panic!(
                "Missing config for environment variable WEB_SERVER_PORT. {}",
                e
            ),
        },
        Err(e) => panic!(
            "Missing config for environment variable WEB_SERVER_HOST. {}",
            e
        ),
    }
}
