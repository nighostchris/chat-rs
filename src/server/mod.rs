use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router, Server};
use dotenvy::var;
use serde::Serialize;
use sqlx::{Pool, Postgres};
use std::net::SocketAddr;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;

#[derive(Serialize)]
struct BaseResponse {
    success: bool,
}

pub struct ServerState {
    db: Pool<Postgres>,
}

// Initialize an axum web server instance
#[tracing::instrument]
pub async fn init(db_client: Pool<Postgres>) {
    let server_state = Arc::new(ServerState { db: db_client });
    // https://stackoverflow.com/questions/74302133/how-to-log-and-filter-requests-with-axum-tokio
    let service = ServiceBuilder::new().layer(TraceLayer::new_for_http());
    let server = Router::new()
        .route("/", get(health_check))
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

#[tracing::instrument]
async fn health_check() -> impl IntoResponse {
    info!("received request");
    (StatusCode::OK, Json(BaseResponse { success: true }))
}
