"use strict";


window.addEventListener("load", main);

function main(){
	let loginForm = document.getElementById("login");
	loginForm.hidden = false;
	loginForm.addEventListener("submit", start);
	let hostInput = document.getElementById("hostinput");
	if (hostInput.value === hostInput.defaultValue) {
		hostInput.value = `ws://${window.location.hostname || "localhost"}:9431`;
	}
}



function start(e) {
	let form = e.target;
	let username = form.username.value;
	let host = form.host.value;
	
	let canvas = document.getElementById("canvas");

	let fuzzTemplate = new FuzzTemplate(document.getElementById("fuzz-template"), 1, 1);

	let spritemap = new SpriteMap();
	spritemap.addSprites(
		document.getElementById("spritemap"),
		{
			player: {x: 0, y: 0, layer: "creatures"},
			sage: {x: 1, y: 0},
			worktable: {x: 6, y: 0},
			altar: {x: 7, y: 0},
			grass1: {x: 0, y: 1, layer: "ground"},
			grass2: {x: 1, y: 1, layer: "ground"},
			grass3: {x: 2, y: 1, layer: "ground"},
			dirt: {x: 3, y: 1, layer: "ground"},
			rockmid: {x: 4, y: 1, border: "#222", layer: "base"},
			" ": {x: 4, y: 1},
			rock: {x: 5, y: 1, border: "#222", layer: "base"},
			water: {x: 6, y: 1, border: "#004", layer: "base"},
			moss: {x: 7, y: 1, layer: "ground"},
			deadleaves: {x: 0, y: 2, layer: "ground"},
			densegrass: {x: 1, y: 2, layer: "ground"},
			wall: {x: 2, y: 2, border: "#222", layer: "base"},
			woodwall: {x: 3, y: 2, border: "#220", layer: "base"},
			stonefloor: {x: 4, y: 2, layer: "base"},
			rockfloor: {x: 5, y: 2, layer: "ground"},
			rush: {x: 0, y: 3},
			pitcherplant: {x: 1, y: 3},
			tree: {x: 2, y: 5, ho: true},
			oldtree: {x: 3, y: 5},
			oldtreetinder: {x: 4, y: 5, ho: true},
			youngtree: {x: 1, y: 5},
			sapling: {x: 0, y: 5},
			shrub: {x: 6, y: 3},
			bush: {x: 7, y: 3},
			reed: {x: 2, y: 3},
			gravel: {x: 5, y: 3},
			pebble: {x: 0, y: 6},
			stone: {x: 1, y: 6},
			stick: {x: 2, y: 6},
		},
		8,
		fuzzTemplate
	);
	let client = new Client(username, host, new Display(canvas, spritemap, fuzzTemplate.asSprite()));
	client.start()
	form.hidden = true;
	window.game_client_debug = client;
}

class Movement {

	constructor() {
		this.keys = {
			KeyW: 0,
			ArrowUp: 0,
			KeyS: 0,
			ArrowDown: 0,
			KeyA: 0,
			ArrowLeft: 0,
			KeyD: 0,
			ArrowRight: 0
		};
	}

	keydown(code) {
		if (this.keys[code] !== undefined) {
			this.keys[code] = 1;
			return true;
		} else {
			return false;
		}
	}

	keyup(code) {
		if (this.keys[code] !== undefined) {
			this.keys[code] = 0;
			return true;
		} else {
			return false;
		}
	}

	clear() {
		for (let key in this.keys) {
			this.keys[key] = 0;
		}
	}

	movement() {
		let right = this.keys.KeyD || this.keys.ArrowRight;
		let left = this.keys.KeyA || this.keys.ArrowLeft;
		let up = this.keys.KeyW || this.keys.ArrowUp;
		let down = this.keys.KeyS || this.keys.ArrowDown;
		return [right - left, down - up];
	}
}


class Client {
	constructor(username, host, display) {
		this.username = username;
		this.host = host;
		this.display = display;
		this.websocket = null;
		this.delay = parseParameters().delay|0;
		this.movement = new Movement();
	}
	
	start(){
		console.log("connecting to '" + this.host + "' as '" + this.username + "'");
		this.websocket = new WebSocket(this.host);
		this.websocket.addEventListener("open", e => {
			document.getElementById("game").hidden = false;
			e.target.send(JSON.stringify({introduction: this.username}));
		});
		let keymap = {
			Period: {select: "next"},
			Comma: {select: "previous"},
			NumpadAdd: {select: "next"},
			NumpadSubtract: {select: "previous"},
			Equal: {select: "next"},
			Minus: {select: "previous"},
		};
		document.addEventListener("keydown", e => {
			if (document.activeElement.classList.contains("captureinput")){
				this.stop()
				if (e.code == "Escape") {
					document.activeElement.blur();
				}
				return;
			}
			if (this.movement.keydown(e.code)) {
				e.preventDefault();
				this.sendInput({movement: this.movement.movement()});
				return;
			}
			let action = keymap[e.code];
			if (action){
				e.preventDefault();
				this.sendInput(action);
			} else {
				if (e.code == "Enter" || e.code == "KeyT") {
					e.preventDefault();
					this.stop();
					document.getElementById("textinput").focus()
				}
			}
		});
		document.addEventListener("keyup", e => {
			if (this.movement.keyup(e.code)) {
				e.preventDefault();
				this.sendInput({movement: this.movement.movement()});
			}
		});
		document.addEventListener("blur", e => this.stop())
		document.getElementById("control-up").addEventListener("click", e => {
			this.sendInput({move: "north"});
		});
		document.getElementById("control-left").addEventListener("click", e => {
			this.sendInput({move: "west"});
		});
		document.getElementById("control-right").addEventListener("click", e => {
			this.sendInput({move: "east"});
		});
		document.getElementById("control-down").addEventListener("click", e => {
			this.sendInput({move: "south"});
		});
		this.websocket.addEventListener("error", console.error);
		if (this.delay) {
			this.websocket.addEventListener("message", msg => setTimeout(() => this.handleMessage(msg), this.delay));
		} else {
			this.websocket.addEventListener("message", msg => this.handleMessage(msg));
		}
		document.getElementById("chatinput").addEventListener("submit", e => {
			let inp = e.target.command;
			this.onCommand(inp.value)
			inp.value = "";
			document.activeElement.blur();
		});
		this.resize();
		window.addEventListener('resize', e => this.resize());
	}

	stop() {
		this.movement.clear();
		this.sendInput({movement: this.movement.movement()});
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
		if (type === "viewarea") {
			this.display.setViewArea(args.area);
		} else if (type === "section") {
			this.display.drawSection(args.area.w, args.area.h, args.area.x, args.area.y, args.field, args.mapping);
		} else if (type === "changecells") {
			this.display.changeTiles(args);
		} else if (type === "dynamics") {
			this.display.drawDynamics(args);
		} else if (type == "playerpos") {
			this.display.setCenter(args[0], args[1]);
			let x = ((args[0] * 100) | 0) / 100;
			let y = ((args[1] * 100) | 0) / 100;
			document.getElementById("coordinates").textContent = `${x}, ${y}`;
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
		let table = document.getElementById("inventory");

		let rows = table.querySelectorAll("li");
		rows.forEach(function(row) {
			row.remove();
		});

		for (let i=0; i<items.length; ++i) {
			let item = items[i];
			let name = item[0];
			let quantity = item[1];
			let row = document.createElement("li");
			row.onclick = e => {
				this.sendInput({select: {idx: i | 0}});
			}
			row.className = "inv-row";

			let nm = document.createElement("span");
			nm.className = "inventory-name";
			nm.innerText = name;
			row.appendChild(nm);

			let am = document.createElement("span");
			am.className = "inventory-amount";
			if (quantity !== null && quantity !== undefined) {
				am.innerText = quantity;
			}
			row.appendChild(am);

			if (i === selected) {
				// nm.className += " inv-selected";
				// am.className += " inv-selected";
				row.className += " inv-selected";
			};
			table.appendChild(row);
			if (Math.abs(i - selected) <= 1) {
				row.scrollIntoView();
			}
		}
	}

	sendInput(msg) {
		let now = Date.now();
		let f = () => {
			if (this.websocket.readyState === WebSocket.OPEN){
				this.websocket.send(JSON.stringify({input: [msg, now]}));
			} else {
				console.error("can't send input: websocket not open", this.websocket.readyState,  msg);
			}
		};
		if (this.delay) {
			setTimeout(f, this.delay);
		} else {
			f();
		}
	}
	
	print(msg, type) {
		console.log("msg", msg);
		let li = document.createElement("li");
		li.innerText = msg;
		let messages = document.getElementById("messages");
		let isAtBottom = messages.lastElementChild && messages.scrollTop + messages.clientHeight >= messages.scrollHeight - messages.lastElementChild.scrollHeight;
		messages.appendChild(li);
		if (isAtBottom){
			li.scrollIntoView();
		}
	}
	
	onCommand(command) {
		this.websocket.send(JSON.stringify({chat: command}));
	}

	resize() {
		this.zooms = this.zooms || 0
		this.zooms += 1
		this.print("zoom " + this.zooms);
		this.display.resize(window.innerWidth, window.innerHeight);
	}
}

function parseParameters(){
	let ps = new URLSearchParams(window.location.search)
	let parameters = {};
	for (let p of ps){
		parameters[p[0]] = p[1];
	}
	return parameters;
}
