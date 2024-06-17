mod config;
mod ctx;
mod error;
mod log;
mod model;
mod web;

use axum::{
    http::{Method, Uri},
    middleware,
    response::{IntoResponse, Response},
    Json, Router,
};

use serde_json::json;
use tower_cookies::CookieManagerLayer;
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

use ctx::Ctx;
use error::{Result, ServerError};
use model::ModelController;
use web::{auth, hello_router, login_router, static_router, tickets_router};

const DEFAULT_IP: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "8080";

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time() // TODO: remove later
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

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
    let addr = listener.local_addr().unwrap();
    info!("{:<12} - {addr}\n", "LISTENING");
    axum::serve(listener, router).await.unwrap();

    Ok(())
}

async fn response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    resp: Response,
) -> Response {
    debug!("{:<12} - main_response_mapper", "MAP_RESPONSE");

    let uuid = Uuid::new_v4();
    let service_error = resp.extensions().get::<ServerError>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body =
                json!({"error": {"type": client_error.as_ref(), "reqUuid": uuid.to_string()}});

            debug!(
                "{:<12} - client_error_body: {client_error_body}",
                "CLIENT_ERROR"
            );

            (*status_code, Json(client_error_body)).into_response()
        });

    let client_error = client_status_error.unzip().1;
    let _ = log::log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    debug!("\n"); // TODO: delete me
    error_response.unwrap_or(resp)
}
