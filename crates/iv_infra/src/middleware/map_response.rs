use std::sync::Arc;

use axum::{
  Json,
  body::{self, to_bytes},
  http::{Method, StatusCode, Uri},
  response::{IntoResponse, Response},
};
use iv_core::errors::{AppError, ClientError};
use serde_json::{Value, json, to_value};
use tracing::debug;
use uuid::Uuid;

pub async fn mw_map_response(uri: Uri, req_method: Method, res: Response) -> Response {
  let uuid = Uuid::new_v4();

  let web_error = res.extensions().get::<Arc<AppError>>().map(Arc::as_ref);
  let client_status_error = web_error.map(|e| e.status_and_error());

  match client_status_error {
    Some((status_code, client_error)) => {
      let client_error = to_value(client_error).ok();
      let message = client_error.as_ref().and_then(|v| v.get("message"));
      let details = client_error.as_ref().and_then(|v| v.get("details"));

      let error_body = json!({
          "data": {
              "req_id": uuid.to_string(),
              "details": details,
              "message": message,
          },
          "status": 0
      });
      (status_code, Json(error_body)).into_response()
    },
    None => {
      let status = res.status();
      let body = to_bytes(res.into_body(), usize::MAX).await.unwrap_or_default();
      let body_string = String::from_utf8(body.to_vec()).unwrap_or_default();
      let data: Value = serde_json::from_str(&body_string).unwrap_or(Value::Null);
      let new_response = json!({
          "req_id": uuid.to_string(),
        "data": data,
        "status": 1,
        "metadata": 0 //pagination
      });
      (status, Json(new_response)).into_response()
    },
  }
}
