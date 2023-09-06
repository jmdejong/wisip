

class FuzzTemplate {

	constructor(image, marginX, marginY) {
		this.image = image;
		this.marginX = marginX;
		this.marginY = marginY;
	}

	fuzz(sprite) {
		console.assert(sprite.width === this.image.width - 2 * this.marginX, "sprite width does not match");
		console.assert(sprite.height === this.image.height - 2 * this.marginY, "sprite height does not match");
		let outImg = document.createElement("canvas");
		outImg.width = this.image.width;
		outImg.height = this.image.height;
		let ctx = outImg.getContext("2d");
		for (let x=this.marginX - sprite.width; x < this.image.width; x += sprite.width) {
			for (let y = this.marginY - sprite.height; y < this.image.height; y += sprite.height){
				sprite.drawOn(ctx, x, y);
			}
		}
		ctx.globalCompositeOperation = "destination-in"
		ctx.drawImage(this.image, 0, 0);
		return new Sprite(outImg, 0, 0, outImg.width, outImg.height, this.marginX, this.marginY);
	}

	asSprite() {
		return new Sprite(this.image, 0, 0, this.image.width, this.image.height, this.marginX, this.marginY);
	}
}
