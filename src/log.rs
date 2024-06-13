use std::time::{SystemTime, UNIX_EPOCH};

use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::{ctx::Ctx, error::ClientError, Result, ServerError};

#[derive(Serialize)]
#[skip_serializing_none]
struct RequestLogLine {
    uuid: String,
    timestamp: String,

    user_id: Option<u64>,

    req_method: String,
    req_path: String,

    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}

pub async fn log_request(
    uuid: Uuid,
    method: Method,
    uri: Uri,
    ctx: Option<Ctx>,
    service_error: Option<&ServerError>,
    client_error: Option<ClientError>,
) -> Result<()> {
    // TODO: handle error
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let error_type = service_error.map(|se| se.as_ref().to_string());
    let error_data = serde_json::to_value(service_error)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(|v| v.take()));

    let log_line = RequestLogLine {
        uuid: uuid.to_string(),
        timestamp: timestamp.to_string(),

        user_id: ctx.map(|c| c.user_id()),

        req_path: uri.to_string(),
        req_method: method.to_string(),

        client_error_type: client_error.map(|e| e.as_ref().to_string()),
        error_type,
        error_data,
    };

    // TODO: send to some log searching service
    println!("request: \n{}", json!(log_line));

    Ok(())
}
