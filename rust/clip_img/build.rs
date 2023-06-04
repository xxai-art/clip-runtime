#[macro_use]
extern crate build_cfg;

use std::process::Command;

#[build_cfg_main]
fn main() -> anyhow::Result<()> {
  if build_cfg!(all(target_os = "macos")) {
    let prefix = Command::new("brew").arg("--prefix").output()?.stdout;
    let prefix = std::str::from_utf8(&prefix)?.trim();
    println!(r"cargo:rustc-link-search={}/lib", prefix);
  }
  Ok(())
}
