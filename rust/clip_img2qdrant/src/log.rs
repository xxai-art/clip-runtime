use tracing_subscriber::{fmt::format::Writer, layer::SubscriberExt, EnvFilter};

struct NoTime;

impl tracing_subscriber::fmt::time::FormatTime for NoTime {
  fn format_time(&self, _writer: &mut Writer<'_>) -> std::fmt::Result {
    Ok(())
  }
}

pub fn init() {
  let env_filter = EnvFilter::from_default_env();
  {
    use tracing_subscriber::util::SubscriberInitExt;
    let fmt = tracing_subscriber::fmt::layer().with_timer(NoTime);
    tracing_subscriber::registry()
      .with(fmt)
      .with(env_filter)
      .init();
  }
}
