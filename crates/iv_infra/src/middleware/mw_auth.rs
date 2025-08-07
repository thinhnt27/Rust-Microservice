use axum::{extract::Request, middleware::Next, response::Response};
use iv_core::AppResult;
use tracing::debug;

pub async fn mw_auth(req: Request, next: Next) -> AppResult<Response> {
    debug!("Authenticating request: {}", req.uri());
    Ok(next.run(req).await)
}