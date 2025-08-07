use axum::{extract::{Path, State}, Json, Router};
use iv_core::AppResult;
use iv_domain::user::{request::RequestGetUser, User};
use sqlx::PgPool;

pub fn get_user_route() -> Router<PgPool> {
    pub async  fn get_user_by_id(State(db): State<PgPool>, Path(id): Path<RequestGetUser>) -> AppResult<Json<User>> {
        iv_infra::user::get_user(State(db), Path(id)).await
    }

    Router::new()
        .route("/user/{id}", axum::routing::get(get_user_by_id))
}