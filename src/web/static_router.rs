use axum::{routing, Router};
use tower_http::services::ServeDir;

pub fn new() -> Router {
    Router::new().nest_service("/", routing::get_service(ServeDir::new("./layout")))
}
