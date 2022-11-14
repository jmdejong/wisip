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
	canvas.hidden = false;
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
			e.target.send(JSON.stringify({introduction: this.username}));
		});
		this.websocket.addEventListener("error", console.error);
		this.websocket.addEventListener("message", console.log);
	}
}
