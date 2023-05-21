
use crate::{
	pos::{Pos},
	random::{WhiteNoise, randomize_u32}
};



pub trait HeightMap {
	fn height(&self, pos: Pos) -> f32;
}

pub struct LazyHeightMap {
	seed: u32,
	depth: Vec<(i32, f32)>,
}

impl LazyHeightMap {

	pub fn new(seed: u32, depth: Vec<(i32, f32)>) -> Self {
		Self{
			seed,
			depth,
		}
	}

	pub fn gen_f(&self, pos: Pos) -> f32 {
		let mut seed = self.seed;
		let n = self.depth.iter()
			.map(|(d, weight)| {
				seed = randomize_u32(seed);
				let c_n = self.gen_depth(pos, *d, seed);
				c_n * weight
			})
			.sum();
		n
	}

	fn gen_depth(&self, pos: Pos, factor: i32, seed: u32) -> f32 {
		if factor == 1 {
			return WhiteNoise::new(seed).gen_f(pos);
		}
		let pos_base = (pos / factor) * factor;
		let diff = pos - pos_base;
		let (u, v) = (diff.x as f32 / factor as f32, diff.y as f32 / factor as f32);
		[
				((0, 0), (1.0 - u) * (1.0 - v)),
				((0, 1), (1.0 - u) * v),
				((1, 0), u * (1.0 - v)),
				((1, 1), u * v)
		].into_iter().map(|((cx, cy), f)| {
			let c = Pos::new(cx, cy) * factor;
			WhiteNoise::new(seed).gen_f(pos_base + c) * f
		}).sum::<f32>()
	}

	// pub fn cache(&self, request_area: Area) -> CachedHeightMap {
	// 	let area = request_area.grow(2);
	// 	let mut seed = self.seed;
	// 	let grids = self.depth.iter().map(|(d, weight)| {
	// 		seed = randomize_u32(seed);
	// 		let noise = WhiteNoise::new(seed);
	// 		let start = area.min() / *d;
	// 		let end = area.max() / *d + Pos::new(1, 1);
	// 		let da = Area::new(start, end - start).grow(1);
	// 		let mut grid = Grid::new(da, 0.0);
	// 		for dp in Area::new(start, end - start).iter() {
	// 			grid.set(dp, noise.gen_f(dp * *d) * weight);
	// 		}
	// 		(*d, grid)
	// 	}).collect();
	// 	CachedHeightMap(grids)
	// }
}

impl HeightMap for LazyHeightMap {
	fn height(&self, pos: Pos) -> f32 {
		self.gen_f(pos)
	}
}

// pub struct CachedHeightMap(Vec<(i32, Grid<f32>)>);
//
// impl HeightMap for CachedHeightMap {
// 	fn height(&self, pos: Pos) -> f32 {
// 		self.0.iter().flat_map(|(d, g)| {
// 			let pb = pos / *d;
// 			let diff = pos - pb * *d;
// 			let (u, v) = (diff.x as f32 / *d as f32, diff.y as f32 / *d as f32);
// 			[
// 					((0, 0), (1.0 - u) * (1.0 - v)),
// 					((0, 1), (1.0 - u) * v),
// 					((1, 0), u * (1.0 - v)),
// 					((1, 1), u * v)
// 			].into_iter().map(move |((cx, cy), f)| {
// 				let c = Pos::new(cx, cy);
// 				// println!("- {:?}, {:?}", pb + c, g.area);
// 				g.get(pb + c).unwrap() * f
// 			})
// 		}).sum::<f32>()
// 	}
// }
