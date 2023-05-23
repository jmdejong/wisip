"use strict";

class Sprite {
	constructor(image, x, y, width, height, originX, originY) {
		this.image = image;
		this.x = x || 0;
		this.y = y || 0;
		this.width = width || image.width;
		this.height = height || image.height;
		this.originX = originX || 0;
		this.originY = originY || 0;
	}

	drawOn(ctx, x, y) {
		ctx.drawImage(this.image, this.x, this.y, this.width, this.height, x - this.originX, y - this.originY, this.width, this.height);
	}
}

class LayeredSprite {
	constructor(layers, border) {
		this.layers = layers;
		this.border = border;
	}
}


class SpriteMap {
	constructor() {
		this.sprites = {};
	}
	
	addSprites(image, mapping, size, fuzzTemplate) {
		for (let name in mapping) {
			let entry = mapping[name];
			let layers = {};
			let mainSprite = new Sprite(image, entry.x * size, entry.y * size, size, size)
			layers[entry.layer || "main"] = mainSprite;
			if (entry.ho) {
				layers.ho = new Sprite(image, entry.x * size, (entry.y - 1) * size, size, size);
			}
			if (entry.layer == "ground") {
				layers.fuzz = fuzzTemplate.fuzz(mainSprite);
			}

			this.sprites[name] = new LayeredSprite(layers, entry.border);
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
		this.layers = ["ground", "fuzz", "base", "borders", "main", "ho"];
		this.buffers = {};
		this.ctxs = {};
		this.spritemap = spritemap;
		this.offsetX = 0;
		this.offsetY = 0;
		this.centerX = 0;
		this.centerY = 0;
		this.borders = [];
		this.width = 0;
		this.height = 0;
		this.scale = 4;
		this.init = false;
	}

	setViewArea(area){
		for (let layer of this.layers) {
			let buffer = document.createElement("canvas");
			buffer.width = area.w * this.tileSize;
			buffer.height = area.h * this.tileSize;
			this.buffers[layer] = buffer;
			let ctx = buffer.getContext("2d");
			this.ctxs[layer] = ctx;
		}
		this.borders = [];
		this.offsetX = area.x;
		this.offsetY = area.y;
		this.width = area.w;
		this.height = area.h;
	}

	drawSection(width, height, offsetX, offsetY, cells, mapping){
		let borderMap = {};
		for (let key in mapping) {
			borderMap[key] = this._border(mapping[key]);
		}
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
		this.init = true
	}

	changeTiles(tiles) {
		if (!this.init) {
			return;
		}
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
		let x = (tileX - this.offsetX) * this.tileSize;
		let y = (tileY - this.offsetY) * this.tileSize;
		let hoY = y;// - this.tileSize;
		// this.ctxs.ground.clearRect(x, y, this.tileSize, this.tileSize);
		this.ctxs.base.clearRect(x, y, this.tileSize, this.tileSize);
		this.ctxs.main.clearRect(x, y, this.tileSize, this.tileSize);
		this.ctxs.ho.clearRect(x, hoY, this.tileSize, this.tileSize);
		for (let i=sprites.length; i --> 0;) {
			let name = sprites[i];
			let sprite = this.spritemap.sprite(name);
			if (sprite) {
				for (let layer in sprite.layers) {
					sprite.layers[layer].drawOn(this.ctxs[layer], x, y);
				}
			} else {
				this.ctxs.base.fillStyle = this._getColor(name);
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
		if (!this.init) {
			return;
		}
		let tileSize = this.tileSize * this.scale;
		let centerX = (this.centerX - this.offsetX) * tileSize;
		let centerY = (this.centerY - this.offsetY) * tileSize;
		// let srcX = Math.max(0, Math.min(this.buffer.width, this.
		this.outerCtx.imageSmoothingEnabled = false;
		for (let layer of this.layers) {
			let buffer = this.buffers[layer];
			this.outerCtx.drawImage(
				buffer,
				this.canvas.width / 2 - centerX,
				this.canvas.height / 2 - centerY - (layer === "ho" ? tileSize : 0),
				buffer.width * this.scale,
				buffer.height * this.scale
			);
		}
	}

	resize(width, height) {
		this.canvas.width = width;;
		this.canvas.height = height;
		this.redraw();
	}
}
