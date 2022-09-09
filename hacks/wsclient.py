#!/usr/bin/env python3

import websockets
import sys
import asyncio
import threading
import json

async def main():

	async with websockets.connect("ws://127.0.0.1:9232") as sock:
		
		#async def listen():
			#while(True):
				#print(await sock.recv())
		
		#threading.Thread(target=listen, daemon=True).start()
		await sock.send(json.dumps(["chat", "abc"]))
		
		for line in sys.stdin:
			await sock.send(json.dumps(["chat", line.strip()]))

asyncio.run(main())
