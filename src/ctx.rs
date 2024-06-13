use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::request::Parts};
use tracing::debug;

use crate::{Result, ServerError};

#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: u64,
}

impl Ctx {
    pub fn new(user_id: u64) -> Self {
        Self { user_id }
    }

    pub fn user_id(&self) -> u64 {
        self.user_id
    }
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = ServerError;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self> {
        debug!("{:<12} - context", "CONTEXT");

        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(ServerError::NoContext)?
            .clone()
    }
}
