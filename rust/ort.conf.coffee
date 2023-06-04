#!/usr/bin/env coffee

> zx/globals:
  path > join
  @w5/uridir
  fs > existsSync
  @iarna/toml:TOML
  @w5/write
  @w5/read

ROOT = uridir(import.meta)

cd ROOT

await $"./sh/onnx_install.sh"
{stdout} = await $"./sh/onnx_available_providers.py"


features = [
  'load-dynamic',
]

li = JSON.parse stdout

feature = li[0].slice(0,-17).toLowerCase()
if feature != 'cpu'
  features.push feature

write(
  join ROOT, 'clip_runtime/src/providers.rs'
  """use ort::ExecutionProvider;

  pub fn providers() -> [ExecutionProvider; 1] { [ExecutionProvider::#{feature}()] }"""
)

CARGO_TOML = 'Cargo.toml'

CARGO_MAKE = 'Cargo.make.toml'

toml = read join ROOT, CARGO_TOML

for dir from TOML.parse(toml).workspace.members
  make_fp = join dir, CARGO_MAKE
  if existsSync make_fp

    fp = join dir, CARGO_TOML
    if existsSync fp
      o = TOML.parse read fp
      delete o.dependencies.ort.features
      write make_fp, TOML.stringify o

    make = read make_fp
    o = TOML.parse make
    o.dependencies.ort.features = features
    write fp, TOML.stringify o
    # console.log features
    # console.log TOML.patch(
    #   make
    #   o
    # )
  # if existsSync fp
  #   console.log fp


#   toml = read fp
#   o = TOML.parse toml
#   if o.dependencies?.ort
#     console.log fp
#
