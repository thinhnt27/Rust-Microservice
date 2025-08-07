use std::sync::Arc;

use axum::{extract::{Path, Request}, http::{Method, StatusCode, Uri}, middleware::{self, Next}, response::{IntoResponse, Response}, routing::get, Extension, Json, Router};
use dotenv::dotenv;
use rust_microservice::{configs::ProdConfig, dbs::initialed_db, errors::AppError, AppResult};
use serde_json::json;
use tracing::info;
use tracing_subscriber::layer;
use uuid::Uuid;

#[tokio::main]
async fn main() {
  dotenv().ok();
  tracing_subscriber::fmt::init();

  let cfg = ProdConfig::from_env().expect("Cann't get env");
  let pool = initialed_db(&cfg.postgres.dsn, cfg.postgres.max_conns).await;

  let app = Router::new().route("/:msg", get(say_hello))
  .layer(middleware::map_response(mw_map_response))
  .layer(middleware::from_fn_with_state(pool.clone(), mw_auth))
  .with_state(Arc::new(pool));
  info!("Connect Database successfully");

  info!("Server is running on port: {}", cfg.web.addr);
  let listener = tokio::net::TcpListener::bind(cfg.web.addr).await.unwrap();
  axum::serve(listener, app).await.unwrap();
}

pub async fn say_hello(Path(msg): Path<String>) -> AppResult<Json<serde_json::Value>> {
  info!("Say helllo!!!");
  if msg.is_empty() {
    Err(AppError::NotFound)
  } else {
    Ok(Json(json!({"message": format!("Hello, {}!", msg)})))
  }
}

pub async fn mw_map_response(uri: Uri, req_method: Method,res: Response) -> Response {
    let uuid = Uuid::new_v4();
    info!("Request ID: {}", uuid);
    info!("Mapping response");
    info!("Request URI: {}", uri);
    info!("Request Method: {}", req_method);
    (StatusCode::ACCEPTED, res).into_response()
}

pub async fn mw_auth(req: Request, next: Next) -> AppResult<Response> {
    info!("Auth Middleware");
    Ok(next.run(req).await)
}