"use strict";


window.addEventListener("load", main);

function main(){
	let loginForm = document.getElementById("login");
	loginForm.hidden = false;
	loginForm.addEventListener("submit", start);
	let hostInput = document.getElementById("hostinput");
	if (hostInput.value === hostInput.defaultValue) {
		hostInput.value = `ws://${window.location.hostname}:9231`;
	}
}

function start(e) {
	let form = e.target;
	let username = form.username.value;
	let host = form.host.value;
	
	let canvas = document.getElementById("canvas");

	// var input = new Input();
	let spritemap = new SpriteMap(
		document.getElementById("spritemap"),
		{
			player: {x: 0, y: 0},
			sage: {x: 1, y: 0},
			worktable: {x: 6, y: 0},
			altar: {x: 7, y: 0},
			grass1: {x: 0, y: 1, layer: "base"},
			grass2: {x: 1, y: 1, layer: "base"},
			grass3: {x: 2, y: 1, layer: "base"},
			dirt: {x: 3, y: 1, layer: "base"},
			rockmid: {x: 4, y: 1, border: "#222", layer: "base"},
			" ": {x: 4, y: 1},
			rock: {x: 5, y: 1, border: "#222", layer: "base"},
			water: {x: 6, y: 1, border: "#004", layer: "base"},
			moss: {x: 7, y: 1, layer: "base"},
			deadleaves: {x: 0, y: 2, layer: "base"},
			densegrass: {x: 1, y: 2, layer: "base"},
			wall: {x: 2, y: 2, border: "#222", layer: "base"},
			woodwall: {x: 3, y: 2, border: "#220", layer: "base"},
			stonefloor: {x: 4, y: 2, layer: "base"},
			rockfloor: {x: 5, y: 2, layer: "base"},
			rush: {x: 0, y: 3},
			pitcherplant: {x: 1, y: 3},
			tree: {x: 2, y: 3},
			oldtree: {x: 3, y: 3},
			oldtreetinder: {x: 3, y: 4},
			youngtree: {x: 4, y: 3},
			sapling: {x: 5, y: 3},
			shrub: {x: 6, y: 3},
			bush: {x: 7, y: 3},
			reed: {x: 0, y: 4},
			gravel: {x: 0, y: 5},
			pebble: {x: 0, y: 6},
			stone: {x: 1, y: 6},
			stick: {x: 2, y: 6},
		},
		8
	);
	let client = new Client(username, host, new Display(canvas, spritemap));
	client.start()
	form.hidden = true;
}


class Client {
	constructor(username, host, display) {
		this.username = username;
		this.host = host;
		this.display = display;
		this.websocket = null;
	}
	
	start(){
		console.log("connecting to '" + this.host + "' as '" + this.username + "'");
		this.websocket = new WebSocket(this.host);
		this.websocket.addEventListener("open", e => {
			document.getElementById("game").hidden = false;
			e.target.send(JSON.stringify({introduction: this.username}));
		});
		let keymap = {
			KeyW: {move: "north"},
			ArrowUp: {move: "north"},
			KeyS: {move: "south"},
			ArrowDown: {move: "south"},
			KeyA: {move: "west"},
			ArrowLeft: {move: "west"},
			KeyD: {move: "east"},
			ArrowRight: {move: "east"},
			Period: {select: "next"},
			Comma: {select: "previous"},
			NumpadAdd: {select: "next"},
			NumpadSubtract: {select: "previous"},
			Equal: {select: "next"},
			Minus: {select: "previous"},
		};
		let shiftKeymap = {
			KeyW: {interact: "north"},
			ArrowUp: {interact: "north"},
			KeyS: {interact: "south"},
			ArrowDown: {interact: "south"},
			KeyA: {interact: "west"},
			ArrowLeft: {interact: "west"},
			KeyD: {interact: "east"},
			ArrowRight: {interact: "east"},
		}
		document.addEventListener("keydown", e => {
			// console.log(e, e.shiftKey)
			let action = (e.shiftKey && shiftKeymap[e.code]) || keymap[e.code];
			// console.log(action);
			if (action){
				e.preventDefault();
				this.sendInput(action);
			}
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
			for (let message of data[1]) {
				this.handleWorldMessage(message);
			}
			this.display.redraw();
		} else {
			console.log("unknown", data);
		}
	}

	handleWorldMessage(message){
		let type = message[0];
		let args = message[1];
		if (type === "field") {
			this.display.drawField(args.width, args.height, args.offset[0], args.offset[1], args.field, args.mapping);
		} else if (type === "changecells") {
			this.display.changeTiles(args);
			// for (let cell of args){
				// this.display.drawTile(cell[0][0], cell[0][1], cell[1]);
			// }
		} else if (type == "playerpos") {
			this.display.setCenter(args[0], args[1]);
		} else if (type === "inventory") {
			this.setInventory(args[0], args[1]);
		} else if (type === "messages") {
			for (let message of args) {
				this.print(message[1], message[0]);
			}
		} else {
			console.log(type, args);
		}
	}

	setInventory(items, selected) {
		let list = document.getElementById("inventory");
		while (list.hasChildNodes()){
			list.removeChild(list.firstChild);
		}
		for (let i in items) {
			let item = items[i];
			let li = document.createElement("li");
			li.onclick = e => {
				this.sendInput({select: {idx: i | 0}});
			}

			let sel = document.createElement("span");
			sel.className = "inventory-selected";
			if (i == selected) {
				sel.className += " selected";
				sel.innerText = "*";
			};
			li.appendChild(sel);

			let nm = document.createElement("span");
			nm.className = "inventory-name";
			nm.innerText = item[0];
			li.appendChild(nm);

			let am = document.createElement("span");
			am.className = "inventory-amount";
			am.innerText = item[1];
			li.appendChild(am);

			list.appendChild(li);
		}
	}

	sendInput(msg) {
		if (this.websocket.readyState === WebSocket.OPEN){
			this.websocket.send(JSON.stringify({input: msg}));
		} else {
			console.error("can't send input: websocket not open", this.websocket.readyState,  msg);
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
