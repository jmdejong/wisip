#!/usr/bin/env python3
import json

with open("largeworld.json") as f:
	text = f.read()
for i in range(100):
	msg = json.loads(text)
print(msg)
