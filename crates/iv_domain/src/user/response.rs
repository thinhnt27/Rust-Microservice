use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseCreateUser {
  pub id: i64,
  pub username: String,
}
