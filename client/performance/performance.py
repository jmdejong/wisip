#!/usr/bin/env python3
import sys
import os.path
sys.path.append(os.path.join(os.path.dirname(__file__), ".."))
from dezlclient.gameclient import Client
from ratuil.cursedscreen import Screen
from dezlclient.display import Display
from dezlclient import loaders
from dezlclient.common import messages
import signal
import cProfile
import pstats
import json

with open("largeworld.json") as f:
	msg = json.load(f)

message = messages.message_from_json(msg)

screen = Screen()

try:
	screen.initialize_terminal()

	display = Display(screen, loaders.loadCharmap("default"))
	client = Client(display, "me", None, loaders.loadKeybindings("default"), None)
	
	client.display.update()
	
	def update():
		for i in range(10):
			client.update(message)
			client.display.update()
	#update()
	cProfile.run('update()', "updateprofile")
	closeMessage = client.closeMessage
finally:
	## Set everything back to normal
	screen.finalize_terminal()

#if error is not None:
	#raise error
stats = pstats.Stats("updateprofile")
stats.print_stats()

#print(message)
if closeMessage:
	print(closeMessage, file=sys.stderr)
