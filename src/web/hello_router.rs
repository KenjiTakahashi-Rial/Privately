use axum::extract::Path;
use axum::extract::Query;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::routing;
use axum::Router;
use serde::Deserialize;
use tracing::debug;

const DEFAULT_HELLO_NAME: &str = "world";

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

pub fn new() -> Router {
    Router::new()
        .route("/hello", routing::get(hello_handler))
        .route("/hello/:name", routing::get(hello_path_handler))
}

async fn hello_handler(Query(params): Query<HelloParams>) -> impl IntoResponse {
    debug!("{:<12} - hello_handler", "HELLO");
    let name = params.name.as_deref().unwrap_or(DEFAULT_HELLO_NAME);
    hello_response(name)
}

async fn hello_path_handler(Path(name): Path<String>) -> impl IntoResponse {
    debug!("{:<12} - hello_path_handler", "HELLO");
    hello_response(&name)
}

fn hello_response(name: &str) -> impl IntoResponse {
    Html(format!("<strong>Hello, {name}!</strong>").to_string())
}
