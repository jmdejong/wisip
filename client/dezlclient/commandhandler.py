
import json

try:
	import hy
except ImportError as e:
	hy = None
	hyErr = e

class InvalidCommandException(Exception):
	pass


class CommandHandler:
	
	def __init__(self, client):
		self.client = client
		
		self.commands = {
			"input": self.input,
			"move": self.move,
			"say": self.say,
			"chat": self.chat,
			"log": self.log,
			"do": self.do,
			"runinput": self.runInput,
			"eval": self.eval,
			"exec": self.exec,
			"scrollchat": self.scrollChat,
			"json": self.json,
			"j": self.json,
			"ijson": self.ijson,
			"ij": self.ijson,
			"hy": self.hy,
			"select": lambda selector: self.input({"select": selector}),
			"interact": lambda direction: self.input({"interact": direction}),
			"help": self.toggleHelp
		}
		
		self.evalArgs = {
			"self": self,
			"client": self.client,
			"connection": self.client.connection,
			"display": self.client.display,
			"print": self.log
		}
	
	def execute(self, action):
		if action is None:
			return
		if isinstance(action[0], str):
			command = action[0]
			if command in self.commands:
				self.commands[command](*action[1:])
			else:
				raise InvalidCommandException("Invalid command '{}'".format(command))
		else:
			raise Exception("Command should be a string")
	
	
	# Commands
	
	def input(self, action):
		self.client.sendInput(action)
	
	def move(self, direction):
		self.input({"move": direction})
	
	def say(self, text):
		self.input(["say", text])
	
	def pick(self, option):
		self.input(["interact", ["none", "north", "south", "east", "west"], option])
	
	def chat(self, text):
		self.client.sendChat( text)
	
	
	def log(self, text):
		self.client.log(text)
	
	def do(self, actions):
		for action in actions:
			self.execute(action)
	
	def runInput(self, startText=""):
		self.client.inputHandler.startTyping(startText)
	
	def toggleHelp(self, *_):
		self.client.toggleHelp()
	
	def eval(self, text):
		self.log(eval(text, self.evalArgs))
	
	def exec(self, text):
		exec(text, self.evalArgs)
	
	def hy(self, code):
		if hy is None:
			self.log(hyErr)
			return
		expr = hy.read_str(code)
		self.log(hy.eval(expr, self.evalArgs))
	
	def scrollChat(self, lines):
		self.client.display.scrollBack(lines)
	
	def json(self, text):
		self.execute(json.loads(text))
	
	def ijson(self, text):
		self.input(json.loads(text))
	
