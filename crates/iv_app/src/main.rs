
use axum::{middleware::{self}, Router};
use dotenv::dotenv;
use iv_core::configs::ProdConfig;
use iv_infra::{initialed_db, middleware::{map_response, mw_auth}};

#[tokio::main]
async fn main() {
  dotenv().ok();
  tracing_subscriber::fmt::init();

  let cfg = ProdConfig::from_env().expect("Cann't get env");
  let pool = initialed_db(&cfg.postgres.dsn, cfg.postgres.max_conns).await;

  let app = Router::new()
  .merge(iv_api::user_router())
  .layer(middleware::map_response(map_response::mw_map_response))
  .layer(middleware::from_fn_with_state(pool.clone(), mw_auth::mw_auth))
  .with_state(pool);

  let listener = tokio::net::TcpListener::bind(cfg.web.addr).await.unwrap();
  axum::serve(listener, app).await.unwrap();
}



