use std::fmt::Debug;

use axum::http::Request;
use futures::FutureExt;
use tower::Service;
use tracing_subscriber::{fmt::format::Writer, layer::SubscriberExt, EnvFilter};

#[derive(Clone)]
pub struct Log;

impl<S> tower::Layer<S> for Log {
  type Service = ResponseTimeService<S>;

  fn layer(&self, service: S) -> Self::Service {
    ResponseTimeService { inner: service }
  }
}

#[derive(Clone)]
pub struct ResponseTimeService<S> {
  inner: S,
}

impl<S, B> Service<Request<B>> for ResponseTimeService<S>
where
  S: Service<Request<B>>,
  B: Send + Debug,
{
  type Response = S::Response;
  type Error = S::Error;
  type Future = futures::future::Map<
    S::Future,
    impl FnOnce(Result<S::Response, S::Error>) -> Result<S::Response, S::Error>,
  >;

  fn poll_ready(
    &mut self,
    cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Result<(), Self::Error>> {
    self.inner.poll_ready(cx)
  }

  fn call(&mut self, req: Request<B>) -> Self::Future {
    #[cfg(not(feature = "stackdriver"))]
    let start_time: u64 = coarsetime::Clock::now_since_epoch().as_millis();
    #[cfg(not(feature = "stackdriver"))]
    let method = req.method().to_string();
    #[cfg(not(feature = "stackdriver"))]
    let url = req.uri().to_string();

    // #[cfg(feature = "stackdriver")]
    // let user_agent;
    // #[cfg(feature = "stackdriver")]
    // {
    //   let headers = req.headers();
    //   user_agent = match headers.get("user-agent") {
    //     Some(t) => Some(t.to_str().unwrap().to_owned()),
    //     None => None,
    //   };
    // }

    self.inner.call(req).map(move |response_result| {
      // #[cfg(feature = "stackdriver")]
      // {
      //   macro_rules! log {
      //   ($($name:ident),*) => {
      //     tracing::info!(
      //       http_request.request_method = %method,
      //       http_request.request_url = %url,
      //       http_request.latency = %latency,
      //       $(http_request.$name = $name,)*
      //     );
      //    };
      //   }
      //   if let Some(user_agent) = user_agent {
      //     log!(user_agent);
      //   } else {
      //     log!();
      //   }
      // }
      #[cfg(not(feature = "stackdriver"))]
      {
        let latency =
          (coarsetime::Clock::now_since_epoch().as_millis() - start_time) as f32 / 1000.0;
        tracing::info!("{} {} {}s", method, url, latency)
      }
      response_result
    })
  }
}

struct NoTime;

impl tracing_subscriber::fmt::time::FormatTime for NoTime {
  fn format_time(&self, _writer: &mut Writer<'_>) -> std::fmt::Result {
    Ok(())
  }
}

pub fn init() {
  let env_filter = EnvFilter::from_default_env();
  #[cfg(feature = "stackdriver")]
  {
    use tracing_subscriber::Registry;
    let stackdriver = tracing_stackdriver::layer();
    let subscriber = Registry::default().with(env_filter).with(stackdriver);
    tracing::subscriber::set_global_default(subscriber).expect("Can't set logger");
  }

  #[cfg(not(feature = "stackdriver"))]
  {
    use tracing_subscriber::util::SubscriberInitExt;
    let fmt = tracing_subscriber::fmt::layer().with_timer(NoTime);
    tracing_subscriber::registry()
      .with(fmt)
      .with(env_filter)
      .init();
  }
}

// struct TimingMiddleware<S> {
//   inner: S,
// }
//
// impl<S> TimingMiddleware<S> {
//   fn new(service: S) -> Self {
//     Self { inner: service }
//   }
// }
//
// impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for TimingMiddleware<S>
// where
//   S: Service<Request<ReqBody>, Response = Response<ResBody>>,
//   S::Error: Into<crate::BoxError>,
//   S::Future: Send + 'static,
//   ReqBody: Send + 'static,
//   ResBody: Send + 'static,
// {
//   type Response = S::Response;
//   type Error = S::Error;
//   type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
//
//   fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//     self.inner.poll_ready(cx)
//   }
//
//   fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
//     // let start_time = Instant::now();
//     let mut inner = self.inner.clone();
//
//     Box::pin(async move {
//       let response = inner.call(req).await?;
//       // let elapsed = start_time.elapsed();
//       info!("Request duration: {:?}", elapsed);
//       Ok(response)
//     })
//   }
// }
//
// // Middleware layer
// struct TimingLayer;
//
// impl<S> tower::Layer<S> for TimingLayer {
//   type Service = TimingMiddleware<S>;
//
//   fn layer(&self, service: S) -> Self::Service {
//     TimingMiddleware::new(service)
//   }
// }
