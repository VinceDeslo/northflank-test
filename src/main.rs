use axum::{
    routing::get,
    Router,
    response::IntoResponse,
    http::StatusCode,
};
use tower_http::trace::TraceLayer;
use tracing::{info, Level};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .with_timer(tracing_subscriber::fmt::time::ChronoUtc::rfc_3339())
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    info!("Initializing application");

    let app = Router::new()
        .route("/", get(health_check))
        .route("/hello", get(hello_world))
        .layer(TraceLayer::new_for_http());

    info!("Starting server on 0.0.0.0:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> impl IntoResponse {
    info!("Health check requested");
    StatusCode::OK
}

async fn hello_world() -> impl IntoResponse {
    info!("Hello world endpoint called");
    "Hello, World!"
}
