pub fn rerange(c: f32, a: f32, b: f32, y: f32, z: f32) -> f32 {
	(((c - a) * (z - y)) / (b - a)) + y
}

pub fn translate(x: i32, y: i32, width: i32) -> usize {
    ((y * width) + x) as usize
}

pub struct Rectangle<T> {
	pub x: T,
	pub y: T,
	pub width: T,
	pub height: T,
}

impl<T> Rectangle<T> {
	pub fn new(x: T, y: T, width: T, height: T) -> Rectangle<T> {
		Rectangle {
			x: x,
			y: y,
			width: width,
			height: height,
		}
	}
}

pub struct V2<T> {
	pub x: T,
	pub y: T,
}

impl<T> V2<T> {
	pub fn new(x: T, y: T) -> V2<T> {
		V2 {
			x: x,
			y: y,
		}
	}
}
