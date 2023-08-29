#!/usr/bin/env coffee

> zx/globals:
  @w5/uridir
  @w5/write
  fs > copyFileSync existsSync
  path > dirname join
  os > platform arch


ROOT = dirname uridir(import.meta)
DIR = join ROOT, 'clip_runtime'
cd DIR

PROVIDER = do =>
  switch platform()
    when 'darwin'
      if 'arm64' == arch()
        return 'coreml'
    when 'linux'
      if existsSync '/proc/driver/nvidia/version'
        return 'tensorrt,cuda'
  'openvino'

cargo_toml = 'Cargo.toml'
cargo_make = 'make.'+cargo_toml

if existsSync cargo_toml
  await $'cargo remove ort 2>/dev/null|| true'
  from_file = cargo_toml
  to_file = cargo_make
else
  from_file = cargo_make
  to_file = cargo_toml

copyFileSync from_file,to_file

await $"cargo add ort --features=#{PROVIDER},load-dynamic --no-default-features"

mod = PROVIDER.split(',').pop()
write(
  'src/providers/mod.rs'
  """mod #{mod};
pub use #{mod}::providers;"""
)


