"use strict";

class Sprite {
	constructor(image, x, y, width, height, layer, border, ho) {
		this.image = image;
		this.x = x;
		this.y = y;
		this.width = width;
		this.height = height;
		this.layer = layer || "main";
		this.border = border
		this.ho = ho;
	}

	hoY(){
		return this.y - this.height;
	}
}

class SpriteMap {
	constructor(image, mapping, size) {
		this.sprites = {};
		for (let name in mapping) {
			let entry = mapping[name];
			this.sprites[name] = new Sprite(image, entry.x * size, entry.y * size, size, size, entry.layer, entry.border, entry.ho);
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
		this.layers = ["base", "borders", "main", "ho"];
		this.buffers = {};//base: null, borders: null, main: null};
		this.ctxs = {};//base: null, borders: null, main: null};
		this.spritemap = spritemap;
		this.offsetX = 0;
		this.offsetY = 0;
		this.centerX = 0;
		this.centerY = 0;
		this.borders = [];
		this.width = 0;
		this.height = 0;
	}

	drawField(width, height, offsetX, offsetY, cells, mapping){
		for (let layer of this.layers) {
			let buffer = document.createElement("canvas");
			buffer.width = width * this.tileSize;
			buffer.height = height * this.tileSize;
			this.buffers[layer] = buffer;
			let ctx = buffer.getContext("2d");
			this.ctxs[layer] = ctx;
		}
		let borderMap = {};
		for (let key in mapping) {
			borderMap[key] = this._border(mapping[key]);
		}
		this.borders = [];
		this.offsetX = offsetX;
		this.offsetY = offsetY;
		this.width = width;
		this.height = height;
		for (let i=0; i<width * height; ++i){
			let x = i % width;
			let y = i / width | 0;
			this._drawTile(x + offsetX, y + offsetY, mapping[cells[i]]);
			this.borders[i] = borderMap[cells[i]];
		}
		for (let lx=0; lx<width; ++lx) {
			for (let ly=0; ly<height; ++ly) {
				this._drawBorder(lx, ly);
			}
		}
	}

	changeTiles(tiles) {
		for (let tile of tiles){
			let x = tile[0][0];
			let y = tile[0][1];
			let sprites = tile[1];
			this._drawTile(x, y, sprites);
			let lx = x - this.offsetX;
			let ly = y - this.offsetY;
			let i = lx + ly * this.width;
			let border = this._border(sprites);
			if (border !== this.borders[i]) {
				this.borders[i] = this._border(sprites);
				this._drawBorder(lx, ly);
				this._drawBorder(lx+1, ly);
				this._drawBorder(lx-1, ly);
				this._drawBorder(lx, ly+1);
				this._drawBorder(lx, ly-1);
			}
		}
	}

	_drawTile(tileX, tileY, sprites) {
		// console.log(tileX, tileY, sprites)
		let x = (tileX - this.offsetX) * this.tileSize;
		let y = (tileY - this.offsetY) * this.tileSize;
		let hoY = y - this.tileSize;
		this.ctxs.base.clearRect(x, y, this.tileSize, this.tileSize);
		this.ctxs.main.clearRect(x, y, this.tileSize, this.tileSize);
		this.ctxs.ho.clearRect(x, hoY, this.tileSize, this.tileSize);
		for (let i=sprites.length; i --> 0;) {
			let name = sprites[i];
			let sprite = this.spritemap.sprite(name);
			if (sprite) {
				this.ctxs[sprite.layer].drawImage(sprite.image, sprite.x, sprite.y, sprite.width, sprite.height, x, y, this.tileSize, this.tileSize);
				if (sprite.ho) {
					this.ctxs.ho.drawImage(sprite.image, sprite.x, sprite.hoY(), sprite.width, sprite.height, x, hoY, this.tileSize, this.tileSize);
				}
			} else {
				this.ctxs.base.fillStyle = this._getColor(name);
				// if (Math.abs(tileX - this.centerX) < 2 && Math.abs(tileY - this.centerY) < 2) {
					// console.log(name, this.ctx.fillStyle)
				// }
				this.ctxs.base.fillRect(x, y, this.tileSize, this.tileSize);
			}
		}
	}

	_drawBorder(lx, ly) {
		let x = lx * this.tileSize;
		let y = ly * this.tileSize;
		this.ctxs.borders.clearRect(x, y, this.tileSize, this.tileSize);
		let border = this._borderAt(lx, ly);
		if (!border) {
			return;
		}
		this.ctxs.borders.strokeStyle = border;
		// console.log(border);
		// this.ctxs.borders.beginPath();
		if (this._borderAt(lx - 1, ly) !== border) {
			this.ctxs.borders.beginPath();
			this.ctxs.borders.moveTo(x+0.5, y);
			this.ctxs.borders.lineTo(x+0.5, y + this.tileSize);
			this.ctxs.borders.stroke();
		}
		if (this._borderAt(lx, ly - 1) !== border) {
			this.ctxs.borders.beginPath();
			this.ctxs.borders.moveTo(x, y+0.5);
			this.ctxs.borders.lineTo(x + this.tileSize, y+0.5);
			this.ctxs.borders.stroke();
		}
		if (this._borderAt(lx + 1, ly) !== border) {
			this.ctxs.borders.beginPath();
			this.ctxs.borders.moveTo(x + this.tileSize-0.5, y);
			this.ctxs.borders.lineTo(x + this.tileSize-0.5, y + this.tileSize);
			this.ctxs.borders.stroke();
		}
		if (this._borderAt(lx, ly + 1) !== border) {
			this.ctxs.borders.beginPath();
			this.ctxs.borders.moveTo(x, y + this.tileSize-0.5);
			this.ctxs.borders.lineTo(x + this.tileSize, y + this.tileSize-0.5);
			this.ctxs.borders.stroke();
		}
		this.ctxs.borders.stroke();
	}

	_borderAt(x, y) {
		if (x < 0 || y < 0 || x >= this.width || y >= this.height) {
			return null;
		} else {
			return this.borders[x + y * this.width];
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

	_border(spriteNames) {
		for (let spriteName of spriteNames) {
			let sprite = this.spritemap.sprite(spriteName);
			if (sprite && sprite.border) {
				return sprite.border;
			}
		}
		return null;
	}

	redraw(){
		let centerX = (this.centerX - this.offsetX) * this.tileSize;
		let centerY = (this.centerY - this.offsetY) * this.tileSize;
		// let srcX = Math.max(0, Math.min(this.buffer.width, this.
		for (let layer of this.layers) {
			this.outerCtx.drawImage(this.buffers[layer], this.canvas.width / 2 - centerX, this.canvas.height / 2 - centerY);
		}
	}
}
