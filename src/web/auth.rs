use axum::body::Body;
use axum::extract::State;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

use crate::model::ModelController;
use crate::web::AUTH_TOKEN;
use crate::{ctx::Ctx, Result, ServerError};

#[derive(Debug)]
pub struct AuthToken {
    pub user_id: u64,
    pub expiration: String,
    pub signature: String,
}

pub async fn require_auth(
    ctx: Result<Ctx>,
    request: Request<Body>,
    next: Next,
) -> Result<Response> {
    debug!("{:<12} - require_auth", "AUTH");
    ctx?;
    Ok(next.run(request).await)
}

pub async fn ctx_resolver(
    _: State<ModelController>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    debug!("{:<12} - ctx resolve", "CONTEXT");

    let result_auth_token = cookies
        .get(AUTH_TOKEN)
        .map(|c| c.value().to_string())
        .ok_or(ServerError::NoAuthToken)
        .and_then(parse_token);

    let result_ctx = match result_auth_token {
        Ok(auth_token) => {
            // TODO: validate other auth token parts
            Ok(Ctx::new(auth_token.user_id))
        }
        Err(e) => Err(e),
    };

    if result_ctx.is_err() && !matches!(result_ctx, Err(ServerError::NoAuthToken)) {
        cookies.remove(Cookie::from(AUTH_TOKEN));
    }

    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

pub fn parse_token(token: String) -> Result<AuthToken> {
    let (_, user_id, expiration, signature) = regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token)
        .ok_or(ServerError::InvalidAuthToken)?;

    let user_id: u64 = user_id.parse().map_err(|_| ServerError::InvalidAuthToken)?;

    Ok(AuthToken {
        user_id,
        expiration: expiration.to_string(),
        signature: signature.to_string(),
    })
}
