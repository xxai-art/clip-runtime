use std::net::SocketAddr;

use axum::{error_handling::HandleErrorLayer, http::StatusCode, BoxError, Router};
use coarsetime::Duration;
use tower::ServiceBuilder;

use crate::env::{env_default, TO};

const TIMEOUT: u64 = 600;

pub async fn srv(router: Router) {
  let addr = SocketAddr::from(([0, 0, 0, 0], env_default("PORT", 3678)));

  unsafe {
    TO = std::env::var("TO").unwrap();
    tracing::info!("http://{addr} -> {}", TO);
  }

  // https://github.com/tokio-rs/axum/discussions/1383
  let middleware = ServiceBuilder::new()
    .layer(HandleErrorLayer::new(|error: BoxError| async move {
      if error.is::<tower::timeout::error::Elapsed>() {
        Ok(StatusCode::REQUEST_TIMEOUT)
      } else {
        Err((
          StatusCode::INTERNAL_SERVER_ERROR,
          format!("Unhandled internal error: {}", error),
        ))
      }
    }))
    .layer(crate::log::Log)
    .timeout(Duration::from_secs(TIMEOUT).into())
    .layer(ServiceBuilder::new())
    .into_inner();

  axum::Server::bind(&addr)
    .serve(router.layer(middleware).into_make_service())
    .await
    .expect("failed");
}
