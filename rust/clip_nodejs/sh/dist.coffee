#!/usr/bin/env coffee

> zx/globals:
  fs > existsSync readFileSync writeFileSync copyFileSync
  path > join
  fs > rmSync
  @w5/mdt/make.js

cwd = process.cwd()

fp = (p)=>
  join cwd,p

package_json = 'package.json'

await $'bunx cep -w -c coffee -o test -e mjs'

package_json_fp = fp package_json
json = JSON.parse readFileSync(
  package_json_fp
  'utf8'
)

{version} = json

version = version.split('.')
version.push parseInt(version.pop())+1
json.version = version = version.join '.'

writeFileSync(
  package_json_fp
  JSON.stringify json,null,2
)
await $'./test.sh'
await make cwd
tag = 'v'+version

await $'git add -u'
await $"git commit -m '#{tag}'"
await $'./sh/clippy.sh'
await $"git tag #{tag}"
await $'git push origin '+tag

