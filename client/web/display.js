"use strict";

class Sprite {
	constructor(image, x, y, width, height) {
		this.image = image;
		this.x = x;
		this.y = y;
		this.width = width;
		this.height = height;
	}
}

class SpriteMap {
	constructor(image, mapping, size) {
		this.sprites = {};
		for (let name in mapping) {
			let entry = mapping[name];
			this.sprites[name] = new Sprite(image, entry.x * size, entry.y * size, size, size);
		}
	}

	sprite(name) {
		return this.sprites[name];
	}
}

class Display {
	tileSize = 8;

	constructor(canvas, spritemap) {
		this.canvas = canvas;
		this.outerCtx = canvas.getContext("2d");
		this.buffer = null;
		this.ctx = null;
		this.spritemap = spritemap;
		this.offsetX = 0;
		this.offsetY = 0;
		this.centerX = 0;
		this.centerY = 0;
	}

	drawField(width, height, offsetX, offsetY, cells, mapping){
		this.buffer = document.createElement("canvas");
		this.buffer.width = width * this.tileSize;
		this.buffer.height = height * this.tileSize;
		this.ctx = this.buffer.getContext("2d");
		this.offsetX = offsetX;
		this.offsetY = offsetY;
		for (let i=0; i<width * height; ++i){
			let x = i % width;
			let y = i / width | 0;
			this.drawTile(x + offsetX, y + offsetY, mapping[cells[i]]);
		}
	}

	drawTile(tileX, tileY, sprites) {
		// console.log(tileX, tileY, sprites)
		let x = (tileX - this.offsetX) * this.tileSize;
		let y = (tileY - this.offsetY) * this.tileSize;
		if (sprites.length == 0) {
			ctx.clearRect(x, y, this.tileSize, this.tileSize);
		}
		for (let i=sprites.length; i --> 0;) {
			let name = sprites[i];
			let sprite = this.spritemap.sprite(name);
			if (sprite) {
				this.ctx.drawImage(sprite.image, sprite.x, sprite.y, sprite.width, sprite.height, x, y, this.tileSize, this.tileSize);
			} else {
				this.ctx.fillStyle = this._getColor(name);
				if (Math.abs(tileX - this.centerX) < 2 && Math.abs(tileY - this.centerY) < 2) {
					console.log(name, this.ctx.fillStyle)
				}
				this.ctx.fillRect(x, y, this.tileSize, this.tileSize);
			}
		}
	}

	setCenter(x, y) {
		this.centerX = x;
		this.centerY = y;
	}

	_getColor(name){
		var hash = 583;
		for (let i=0; i<name.length; ++i) {
			hash *= 7;
			hash += name.charCodeAt(i);
			hash %= 256 * 256 * 256;
		}
		return "#" + hash.toString(16);
	}

	redraw(){
		let centerX = (this.centerX - this.offsetX) * this.tileSize;
		let centerY = (this.centerY - this.offsetY) * this.tileSize;
		// let srcX = Math.max(0, Math.min(this.buffer.width, this.
		this.outerCtx.drawImage(this.buffer, this.canvas.width / 2 - centerX, this.canvas.height / 2 - centerY);

	}
}
