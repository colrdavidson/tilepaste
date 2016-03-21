#[derive(Debug)]
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

fn rerange(c: f32, a: f32, b: f32, y: f32, z: f32) -> f32 {
	(((c - a) * (z - y)) / (b - a)) + y
}

pub struct Map {
	map: Vec<Tile>,
	pub height: usize,
	pub width: usize,
}

impl Map {
	pub fn new(width: usize, height: usize) -> Map {
		let mut map = Vec::with_capacity((width) * (height));

		for index in 0..map.capacity() {
			let x = index % width;
			let y = index / width;
			let id;
			if (x % 5) == 0 || (y % 5) == 0 {
				id = 3;
			} else if (y % 2) == 0 {
				id = 1;
			} else {
				id = 2;
			}
			map.push(Tile::new(id, x as u32, y as u32));
		}

		Map {
			map: map,
			height: height,
			width: width,
		}
	}

	pub fn uniform(&self, x: u32, y: u32, view_x: u32, view_y: u32, view_width: u32, view_height: u32) -> ([[f32; 4]; 4], u32) {
		let idx = (view_y * (self.width as u32)) + view_x;
		let err_tile = Tile::new(view_x, view_y, 0);

		let tile;
		let tmp = self.map.get(idx as usize);
		if tmp.is_some() {
			tile = tmp.unwrap();
		} else {
			tile = &err_tile;
		}

		let scaled_x = rerange(x as f32, 0.0, ((view_width as f32) - 1.0), -0.8, 0.8);
		let scaled_y = rerange(y as f32, 0.0, ((view_height as f32) - 1.0), -0.8, 0.8);

		let matrix = [
			[1.0, 0.0, 0.0, 0.0],
			[0.0, 1.0, 0.0, 0.0],
			[0.0, 0.0, 1.0, 0.0],
			[scaled_x, scaled_y, 0.0, 1.0f32],
		];
		(matrix, tile.tex_id)
	}

	pub fn size(&self) -> usize {
		self.width * self.height
	}
}

pub struct View {
	pub x: u32,
	pub y: u32,
	pub width: u32,
	pub height: u32,
}

impl View {
	pub fn new(start_x: u32, start_y: u32, width: u32, height: u32) -> View {
		View {
			x: start_x,
			y: start_y,
			width: width,
			height: height,
		}
	}

	pub fn down(&mut self) {
		let new_y = self.y - 1;

		if new_y == 0 || new_y > self.height {
			self.y = 0;
		} else {
			self.y = new_y;
		}
	}

	pub fn up(&mut self) {
		let new_y = self.y + 1;

		if new_y >= self.height {
			self.y = self.height;
		} else {
			self.y = new_y;
		}
	}

	pub fn left(&mut self) {
		let new_x = self.x - 1;

		if new_x == 0 || new_x > self.width {
			self.x = 0;
		} else {
			self.x = new_x;
		}
	}

	pub fn right(&mut self) {
		let new_x = self.x + 1;

		if new_x >= self.width {
			self.x = self.width;
		} else {
			self.x = new_x;
		}
	}
}
