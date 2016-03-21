pub struct Tile {
	tex_id: u32,
	x: u32,
	y: u32,
}

impl Tile {
	pub fn new(id: u32, x: u32, y: u32) -> Tile {
		Tile {
			tex_id: id,
			x: x,
			y: y,
		}
	}
}

pub struct Map {
	map: Vec<Tile>,
	height: usize,
	pub width: usize,
}

impl Map {
	pub fn new(width: usize, height: usize) -> Map {
		let mut map = Vec::with_capacity(width * height);

		for index in 0..map.capacity() {
			let x = index % width;
			let y = index / width;
			map.push(Tile::new(1, x as u32, y as u32));
		}
		Map {
			map: map,
			height: height,
			width: width,
		}
	}

	pub fn uniform(&self, x: usize, y: usize) -> [[f32; 4]; 4] {
		let x = x as f32;
		let y = y as f32;
		let scaled_x = (((x - 0.0) * (1.0 - (-1.0))) / (((self.width as f32) - 0.0) + (-1.0))) - 1.0;
		let scaled_y = (((y - 0.0) * (1.0 - (-1.0))) / (((self.height as f32) - 0.0) + (-1.0))) - 1.0;

		let matrix = [
			[1.0, 0.0, 0.0, 0.0],
			[0.0, 1.0, 0.0, 0.0],
			[0.0, 0.0, 1.0, 0.0],
			[scaled_x, scaled_y, 0.0, 1.0f32],
		];
		matrix
	}

	pub fn size(&self) -> usize {
		self.width * self.height
	}
}
