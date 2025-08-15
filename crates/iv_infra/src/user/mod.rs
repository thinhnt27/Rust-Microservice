use axum::{
  Json,
  extract::{Path, State},
};
use iv_core::{AppResult, errors::AppError};
use iv_domain::user::{
  User,
  request::{RequestCreateUser, RequestGetUser, RequestUpdateUser},
};
use modql::{SIden, field::HasFields};
use sea_query::{Asterisk, IntoIden, PostgresQueryBuilder, Query, TableRef, value};
use sea_query_binder::SqlxBinder;
use sqlx::{FromRow, PgPool, postgres::PgRow};
use tracing::info;

pub trait DMC {
  const SCHEMA: &'static str;
  const TABLE: &'static str;
  fn table_ref() -> TableRef {
    TableRef::SchemaTable(SIden(Self::SCHEMA).into_iden(), SIden(Self::TABLE).into_iden())
  }
}

pub async fn create<MC, I, O>(db: PgPool, input: I) -> AppResult<Json<O>>
where
  MC: DMC,
  I: HasFields,
  O: HasFields + for<'a> FromRow<'a, PgRow> + Send + Unpin,
{
  //Setup data
  let fields = input.not_none_fields();
  let (columns, sea_values) = fields.for_sea_insert();

  //Preparing Query
  let mut query = Query::insert();

  query
    .into_table(MC::table_ref())
    .columns(columns)
    .values(sea_values)?
    .returning(Query::returning().columns([Asterisk]));

  //Execute
  let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
  info!("SQL: {}", sql);
  info!("Values: {:?}", values);

  let entity = sqlx::query_as_with::<_, O, _>(&sql, values).fetch_one(&db).await?;

  // let req = input.into();
  // let (id,) =
  //   sqlx::query_as::<_, (i64,)>(r#"INSERT INTO "user"."tbl_users" (username) VALUES ($1) RETURNING pk_user_id"#)
  //     .bind(req.username)
  //     .fetch_one(&db)
  //     .await?;
  // Ok(Json(id))

  Ok(Json(entity))
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

pub async fn delete_user(db: PgPool, id: i64) -> AppResult<()> {
  let user =
    sqlx::query(r#"DELETE FROM "user"."tbl_users" WHERE pk_user_id = $1"#).bind(id).execute(&db).await?.rows_affected();
  if user == 0 {
    return Err(AppError::NotFound);
  }
  Ok(())
}
