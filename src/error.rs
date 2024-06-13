use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use strum_macros::AsRefStr;

pub type Result<T> = core::result::Result<T, ServerError>;

#[derive(AsRefStr, Clone, Debug, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum ServerError {
    LoginFailed,
    TicketIdNotFound { id: u64 },
    NoAuthToken,
    InvalidAuthToken,
    NoContext,
}

impl ServerError {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            Self::LoginFailed => (StatusCode::FORBIDDEN, ClientError::LoginFailed),
            Self::NoAuthToken | Self::InvalidAuthToken | Self::NoContext => {
                (StatusCode::FORBIDDEN, ClientError::NoAuth)
            }
            Self::TicketIdNotFound { .. } => (StatusCode::BAD_REQUEST, ClientError::InvalidParams),
        }
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        println!("error");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);

        response
    }
}

#[derive(Debug, AsRefStr)]
pub enum ClientError {
    LoginFailed,
    NoAuth,
    InvalidParams,
    ServiceError,
}
