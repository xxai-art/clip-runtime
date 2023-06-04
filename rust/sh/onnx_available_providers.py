#!/usr/bin/env python

import onnxruntime as rt
from json import dumps

print(dumps(rt.get_available_providers()))
