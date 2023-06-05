#!/usr/bin/env node
var ROOT, main;

import uridir from '@w5/uridir';

import read from '@w5/read';

import write from '@w5/write';

import {
  existsSync
} from 'fs';

import {
  join,
  dirname
} from 'path';

ROOT = dirname(uridir(import.meta));

main = () => {
  var end, export_li, i, index, j, len, li, out, p, patch, patch_fp, pos, t;
  index = read(join(ROOT, 'index.js'));
  if (~index.indexOf('import.meta.url')) {
    return;
  }
  li = index.replace(/module\.exports\..*/g, '').trimEnd().split('\n');
  for (pos = j = 0, len = li.length; j < len; pos = ++j) {
    i = li[pos];
    if (i.startsWith('const {')) {
      p = i.indexOf('require(');
      if (p > 0) {
        i = i.trimEnd();
        li[pos] = 'import ' + i.slice(6, -1).replace(/\s*=\s*/g, ' from ').replace('require(', '');
      }
    }
  }
  li = li.join('\n');
  p = li.lastIndexOf('const ');
  if (p > 0) {
    t = li.slice(p);
    li = li.slice(0, p);
    p = t.lastIndexOf('}');
    end = t.slice(p);
    export_li = t.slice(t.indexOf('{') + 1, p).split(',').map((i) => {
      return i.trim();
    });
  }
  out = `import { createRequire } from "module";
import { dirname, sep } from "path";
const __dirname = dirname(decodeURIComponent(import.meta.url.slice(sep=='/'? 7:8)));
const require = createRequire(import.meta.url);` + li;
  patch_fp = join(ROOT, 'patch.js');
  if (existsSync(patch_fp)) {
    patch = read(patch_fp);
    out += ';\n' + patch;
  } else {
    patch = '';
  }
  if (export_li) {
    for (i of export_li) {
      out += '\nexport const ' + i + ' = nativeBinding.' + i + ';';
    }
  }
  write(join(ROOT, 'index.js'), out);
};

main();
