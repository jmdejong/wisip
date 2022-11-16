"use strict";


window.addEventListener("load", main);

function main(){
	let loginForm = document.getElementById("login");
	loginForm.hidden = false;
	loginForm.addEventListener("submit", start);
}

function start(e) {
	let form = e.target;
	let username = form.username.value;
	let host = form.host.value;
	
	let canvas = document.getElementById("canvas");
	
	let client = new Client(username, host, canvas);
	client.start()
	form.hidden = true;
}


class Client {
	constructor(username, host, canvas) {
		this.username = username;
		this.host = host;
		this.canvas = canvas;
		this.websocket = null;
	}
	
	start(){
		console.log("connecting to '" + this.host + "' as '" + this.username + "'");
		this.websocket = new WebSocket(this.host);
		this.websocket.addEventListener("open", e => {
			document.getElementById("game").hidden = false;
			e.target.send(JSON.stringify({introduction: this.username}));
		});
		this.websocket.addEventListener("error", console.error);
		this.websocket.addEventListener("message", msg => this.handleMessage(msg));
		document.getElementById("chat").addEventListener("submit", e => {
			let inp = e.target.command;
			this.onCommand(inp.value)
			inp.value = "";
		});
	}
	
	handleMessage(msg) {
		let data = JSON.parse(msg.data)
		let type = data[0];
		if (type === "message") {
			this.print(data[1]);
		} else if (type === "messages") {
			for (let mesg of data[1]) {
				this.print(data[1], data[0]);
			}
		} else if (type === "world") {
			void(0);
		} else {
			console.log("unknown", data);
		}
	}
	
	print(msg, type) {
		console.log("msg", msg);
		let li = document.createElement("li");
		li.innerText = msg;
		document.getElementById("messages").appendChild(li);
	}
	
	onCommand(command) {
		this.websocket.send(JSON.stringify({chat: command}));
	}
}
