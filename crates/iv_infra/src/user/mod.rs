use axum::{
  Json,
  extract::{Path, State},
};
use iv_core::{AppResult, errors::AppError};
use iv_domain::user::{
  User,
  request::{RequestCreateUser, RequestGetUser, RequestUpdateUser},
};
use sqlx::PgPool;

pub async fn create_user(db: PgPool, req: RequestCreateUser) -> AppResult<Json<i32>> {
  let (id,) =
    sqlx::query_as::<_, (i32,)>(r#"INSERT INTO "user"."tbl_users" (username) VALUES ($1) RETURNING pk_user_id"#)
      .bind(req.username)
      .fetch_one(&db)
      .await?;
  Ok(Json(id))
}
pub async fn get_user(db: PgPool, id: RequestGetUser) -> AppResult<Json<User>> {
  let user = sqlx::query_as::<_, User>(r#"SELECT * FROM "user"."tbl_users" WHERE pk_user_id = $1"#)
    .bind(id.id)
    .fetch_optional(&db)
    .await?
    .ok_or(AppError::NotFound)?;
  Ok(Json(user))
}

pub async fn list(db: PgPool) -> AppResult<Json<Vec<User>>> {
  let users =
    sqlx::query_as::<_, User>(r#"SELECT * FROM "user"."tbl_users" ORDER BY pk_user_id"#).fetch_all(&db).await?;
  Ok(Json(users))
}

pub async fn update_user(db: PgPool, req: RequestUpdateUser) -> AppResult<()> {
  let user = sqlx::query(r#"UPDATE "user"."tbl_users" SET username = $1 WHERE pk_user_id = $2"#)
    .bind(req.username)
    .bind(req.id)
    .execute(&db)
    .await?
    .rows_affected();
  if user == 0 {
    return Err(AppError::NotFound);
  }
  Ok(())
}

pub async fn delete_user(db: PgPool, id: i32) -> AppResult<()> {
  let user =
    sqlx::query(r#"DELETE FROM "user"."tbl_users" WHERE pk_user_id = $1"#).bind(id).execute(&db).await?.rows_affected();
  if user == 0 {
    return Err(AppError::NotFound);
  }
  Ok(())
}
