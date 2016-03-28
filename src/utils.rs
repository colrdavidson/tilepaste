pub fn rerange(c: f32, a: f32, b: f32, y: f32, z: f32) -> f32 {
	(((c - a) * (z - y)) / (b - a)) + y
}

pub fn translate(x: u32, y: u32, width: u32) -> usize {
    ((y * width) + x) as usize
}
