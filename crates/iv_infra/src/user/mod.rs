use axum::{extract::{Path, State}, Json};
use iv_core::{errors::AppError, AppResult};
use iv_domain::user::{request::RequestGetUser, User};
use sqlx::PgPool;

pub async fn get_user(State(db): State<PgPool>, Path(id): Path<RequestGetUser>) -> AppResult<Json<User>> {
  let user = sqlx::query_as::<_, User>(r#"SELECT * FROM "user"."tbl_users" WHERE pk_user_id = $1"#)
    .bind(id.id)
    .fetch_optional(&db)
    .await?.ok_or(AppError::NotFound)?;
  Ok(Json(user))
}