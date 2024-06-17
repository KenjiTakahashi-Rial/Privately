use axum::{
    handler::HandlerWithoutStateExt,
    http::StatusCode,
    routing::{any_service, MethodRouter},
};
use tower_http::services::ServeDir;

use crate::config;

pub fn route_to_dir() -> MethodRouter {
    async fn not_found() -> (StatusCode, &'static str) {
        (StatusCode::NOT_FOUND, "resource not found")
    }

    any_service(
        ServeDir::new(&config::new().layout_dir).not_found_service(not_found.into_service()),
    )
}
