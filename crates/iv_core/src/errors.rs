use std::{env, sync::Arc};

use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
  #[error("Configuration error")]
  Config(#[from] config::ConfigError),

  #[error("Not found...")]
  NotFound,

  #[error("Env Error")]
  EnvError(#[from] env::VarError),

  #[error("Sqlx Error")]
  Sqlx(#[from] sqlx::Error),

  #[error("Sea Error")]
  SeaQuery(#[from] sea_query::error::Error),

  #[error("Entity Not Found")]
  EntityNotFound { entity: &'static str, id: i64 },
}

impl IntoResponse for AppError {
  fn into_response(self) -> axum::response::Response {
    let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
    response.extensions_mut().insert(Arc::new(self));
    response
  }
}

impl AppError {
  pub fn status_and_error(&self) -> (StatusCode, ClientError) {
    use self::AppError::*;
    match self {
      EntityNotFound { entity, id } => (StatusCode::FORBIDDEN, ClientError::EntityNotFound { entity, id: *id }),
      _ => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::ServerError),
    }
  }
}

#[derive(Serialize, Debug)]
#[serde(tag = "message", content = "details")]
pub enum ClientError {
  ServerError,
  EntityNotFound { entity: &'static str, id: i64 },
  NotFound,
}
