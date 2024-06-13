use axum::{routing, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use crate::{web::AUTH_TOKEN, ServerError, Result};

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

pub fn new() -> Router {
    Router::new().route("/api/login", routing::post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("api_login");

    // TODO: authentication

    if payload.username != "ob" || payload.password != "ob" {
        return Err(ServerError::LoginFailed);
    }

    // TODO: generate auth token
    cookies.add(Cookie::new(AUTH_TOKEN, "user-1.expiration.signature"));

    let body = Json(json!({"result": {"success": true}}));

    Ok(body)
}
