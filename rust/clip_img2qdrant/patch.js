const add = nativeBinding.Db.prototype.add;

nativeBinding.Db.prototype.add = function (id, payload, img, ext){
  return add.call(this,id,JSON.stringify(payload),img,ext)
} 

nativeBinding.Db = new Proxy(
  {},
  {
    get:(_,name)=>nativeBinding.dbNew(name)
  }
)



/*
import autoe from "@w5/utf8/autoe.js";
import { u8merge } from "@w5/u8";

const { z85Dump: _z85Dump, zipU64: _zipU64 } = nativeBinding;

const autoeLi = new Proxy(
	{},
	{
		get: (_, name) => {
			const func = nativeBinding[name];
			nativeBinding[name] = (...args) => {
				return func(u8merge(...args.map(autoe)));
			};
			return;
		},
	},
);

autoeLi.passwordHash;
autoeLi.cookieEncode;

nativeBinding.z85Dump = (s) => _z85Dump(autoe(s));
nativeBinding.zipU64 = (...args) => _zipU64(args);
*/

// const _Db = nativeBinding.Db;
//
// nativeBinding.Db = (name)=>
//   new _Db(name)
//
//
