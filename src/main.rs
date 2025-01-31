use axum::{
    routing::get,
    Router,
    response::IntoResponse,
    http::StatusCode,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(health_check))
        .route("/hello", get(hello_world));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

async fn hello_world() -> impl IntoResponse {
    "Hello, World!"
}
