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
