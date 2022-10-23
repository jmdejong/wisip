


pub fn ease_in_out_cubic(x: f32) -> f32 {
	if x < 0.5 {
		4.0 * x.powi(3) 
	} else {
		1.0 - (-2.0 * x + 2.0).powi(3) / 2.0
	}
}

pub fn ease_out_quad(x: f32) -> f32 {
	1.0 - (1.0 - x).powi(2)
}
