use axum::{http::{Method, StatusCode, Uri}, response::{IntoResponse, Response}};
use tracing::debug;
use uuid::Uuid;

pub async fn mw_map_response(uri: Uri, req_method: Method,res: Response) -> Response {
    let uuid = Uuid::new_v4();
    debug!("Request ID: {}, Method: {}, URI: {}", uuid, req_method, uri);
    (StatusCode::ACCEPTED, res).into_response()
}


