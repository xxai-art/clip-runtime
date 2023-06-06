pub fn env_default<T: std::str::FromStr>(env_var: &str, default: T) -> T {
  match std::env::var(env_var) {
    Ok(port) => port.parse().unwrap_or(default),
    Err(_) => default,
  }
}
