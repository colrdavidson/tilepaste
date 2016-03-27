use map::{Tile, View};

pub struct Player {
	pub x: u32,
	pub y: u32,
	pub map_height: u32,
	pub map_width: u32,
	pub tile: Tile,
}

impl Player {
	pub fn new(h: u32, w: u32, x: u32, y: u32, id: u32) -> Player {
		Player {
			x: x,
			y: y,
			map_height: h,
			map_width: w,
			tile: Tile::new(id, x, y),
		}
	}

	pub fn down(&mut self, view: &mut View) {
		if self.tile.y == 0 || (self.tile.y - 1) > self.map_height {
			view.down();
		} else {
			self.tile.y -= 1;
		}

		if self.y > 0 {
			self.y -= 1;
		}
	}

	pub fn up(&mut self, view: &mut View) {
		let new_y = self.tile.y + 1;

		if new_y >= (view.height - 1) {
			self.tile.y = view.height - 1;
			view.up();
		} else {
			self.tile.y = new_y;
		}

		if self.y < view.total_height {
			self.y += 1;
		}
	}

	pub fn left(&mut self, view: &mut View) {
		if self.tile.x == 0 || (self.tile.x - 1) > self.map_width {
			view.left();
		} else {
			self.tile.x -= 1;
		}

		if self.x > 0 {
			self.x -= 1;
		}
	}

	pub fn right(&mut self, view: &mut View) {
		let new_x = self.tile.x + 1;

		if new_x >= (view.width - 1) {
			self.tile.x = view.height - 1;
			view.right();
		} else {
			self.tile.x = new_x;
		}

		if self.x < view.total_width {
			self.x += 1;
		}
	}
}
