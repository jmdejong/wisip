
use crate::Pos;

const MULTIPLIER: u64 = 0x5DEECE66D;
const ADDEND: u64 = 0xB;
const MASK: u64 = (1 << 48) - 1;


pub fn randomize_u32(seed: u32) -> u32 {
	let num: u64 = (seed as u64 ^ MULTIPLIER) & MASK;
	(((num.wrapping_mul(MULTIPLIER).wrapping_add(ADDEND)) & MASK) >> 16) as u32
}


pub fn randomize_pos(pos: Pos) -> u32 {
	randomize_u32(pos.x as u32 ^ randomize_u32(pos.y as u32))
}

pub fn pick<T>(seed: u32, choices: &[T]) -> &T {
	&choices[seed as usize % choices.len()]
}


pub fn pick_weighted<T>(seed: u32, choices: &[(T, u32)]) -> &T {
	let total: u32 = choices.iter().map(|(_v, c)| c).sum();
	let mut rind = (seed % total) as i32;
	for (value, chance) in choices {
		rind -= *chance as i32;
		if rind <= 0 {
			return value;
		}
	}
	panic!("weighted picking exceeds index");
}




#[derive(Debug, Clone)]
pub struct WhiteNoise {
	seed: u32
}

impl WhiteNoise {
	pub fn new(seed: u32) -> Self {
		Self{seed: randomize_u32(seed)}
	}
	
	pub fn gen(&self, pos: Pos) -> u32 {
		randomize_u32(self.seed ^ randomize_pos(pos))
	}
	
	pub fn gen_f(&self, pos: Pos) -> f32 {
		(self.gen(pos) & 0xffff) as f32 / (0x10000 as f32)
	}
}


#[derive(Debug, Clone)]
pub struct Fractal {
	seed: u32,
	depth: Vec<(i32, f32)>,
}

impl Fractal {

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
		})
		.sum::<f32>()
	}
}
