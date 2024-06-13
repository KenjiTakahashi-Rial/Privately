use axum::{
    http::{Method, Uri},
    middleware,
    response::{IntoResponse, Response},
    Json, Router,
};

use ctx::Ctx;
use model::ModelController;
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use uuid::Uuid;
use web::{auth, hello_router, login_router, static_router, tickets_router};

mod ctx;
mod error;
mod log;
mod model;
mod web;

pub use self::error::{Result, ServerError};

const DEFAULT_IP: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "8080";

#[tokio::main]
async fn main() -> Result<()> {
    let controller = ModelController::new().await?;

    let _ = tickets_router::new(controller.clone())
        .route_layer(middleware::from_fn(auth::require_auth));

    let router = Router::new()
        .merge(hello_router::new()) // TODO: delete me
        .merge(login_router::new())
        // .nest("/api", api_routes)
        .layer(middleware::map_response(response_mapper))
        .layer(middleware::from_fn_with_state(
            controller.clone(),
            auth::ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(static_router::new());

    let listener = tokio::net::TcpListener::bind(format!("{DEFAULT_IP}:{DEFAULT_PORT}"))
        .await
        .unwrap();
    tracing::debug!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();

    Ok(())
}

async fn response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    resp: Response,
) -> Response {
    println!("main_response_mapper");

    let uuid = Uuid::new_v4();
    let service_error = resp.extensions().get::<ServerError>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body =
                json!({"error": {"type": client_error.as_ref(), "reqUuid": uuid.to_string()}});

            println!("client_error_body: {client_error_body}");

            (*status_code, Json(client_error_body)).into_response()
        });

    let client_error = client_status_error.unzip().1;
    let _ = log::log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    println!();
    error_response.unwrap_or(resp)
}
