use axum::Router;
use sqlx::PgPool;

use crate::user::{CourseDmc, UserDmc};

pub mod user;

pub fn user_router() -> Router<PgPool> {
  Router::new().nest(
    "/api/v1",
    Router::new()
      .merge(user::get_user_route())
      .merge(user::get_users_route())
      .merge(UserDmc::create_user_route())
      .merge(CourseDmc::create_course_route())
      .merge(user::update_user_route())
      .merge(user::delete_user_route()),
  )
}
