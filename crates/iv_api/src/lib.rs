use axum::Router;
use sqlx::PgPool;

pub mod user;

pub fn user_router() -> Router<PgPool> {
    Router::new()
        .nest("/api/v1", Router::new().merge(user::get_user_route()))
}