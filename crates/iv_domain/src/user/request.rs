use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestGetUser {
  pub id: i32,
}

#[derive(Deserialize)]
pub struct RequestCreateUser {
  pub username: String,
  // pub password: String,
  // pub full_name: Option<String>
}

#[derive(Deserialize)]
pub struct RequestUpdateUser {
  pub id: i32,
  pub username: String,
}
