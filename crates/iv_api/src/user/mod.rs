use axum::{
  Json, Router,
  extract::{Path, State},
};
use iv_core::AppResult;
use iv_domain::user::{
  User,
  request::{RequestCreateUser, RequestGetUser, RequestUpdateUser},
};
use sqlx::PgPool;

pub fn get_user_route() -> Router<PgPool> {
  pub async fn get_user_by_id(State(db): State<PgPool>, Path(id): Path<RequestGetUser>) -> AppResult<Json<User>> {
    iv_infra::user::get_user(db, id).await
  }

  Router::new().route("/user/{id}", axum::routing::get(get_user_by_id))
}

pub fn get_users_route() -> Router<PgPool> {
  pub async fn get_users(State(db): State<PgPool>) -> AppResult<Json<Vec<User>>> {
    iv_infra::user::list(db).await
  }

  Router::new().route("/users", axum::routing::get(get_users))
}

pub fn update_user_route() -> Router<PgPool> {
  pub async fn update_user(State(db): State<PgPool>, Json(req): Json<RequestUpdateUser>) -> AppResult<()> {
    iv_infra::user::update_user(db, req).await
  }

  Router::new().route("/user", axum::routing::put(update_user))
}

pub fn create_user_route() -> Router<PgPool> {
  pub async fn create_user(State(db): State<PgPool>, Json(req): Json<RequestCreateUser>) -> AppResult<Json<i32>> {
    iv_infra::user::create_user(db, req).await
  }

  Router::new().route("/user", axum::routing::post(create_user))
}

pub fn delete_user_route() -> Router<PgPool> {
  pub async fn delete_user(State(db): State<PgPool>, Path(id): Path<i32>) -> AppResult<()> {
    iv_infra::user::delete_user(db, id).await
  }

  Router::new().route("/user/{id}", axum::routing::delete(delete_user))
}
