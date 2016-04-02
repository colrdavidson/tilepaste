use glium;

use player::Player;
use tile::{Tile, TileAtlas};
use utils::{rerange, translate};

pub struct View {
	pub x: i32,
	pub y: i32,
	pub width: i32,
	pub height: i32,
}

impl View {
	pub fn new(start_x: i32, start_y: i32, width: i32, height: i32) -> View {
		View {
			x: start_x,
			y: start_y,
			width: width,
			height: height,
		}
	}
}

pub struct Map<'a> {
	pub map: Vec<Tile<'a>>,
	pub player: Player<'a>,
	pub view: View,
	pub height: i32,
	pub width: i32,
}

impl<'a> Map<'a> {
	pub fn new(width: i32, height: i32, view_width: i32, view_height: i32, atlas: &'a TileAtlas) -> Map<'a> {
		let mut map = Vec::with_capacity((width * height) as usize);

		for index in 0..map.capacity() {
			let x = (index as i32) % width;
			let y = (index as i32) / width;
			let id;
			if (x % 5) == 0 || (y % 5) == 0 {
				id = 10;
			} else if (y % 2) == 0 || (x % 2) == 0 {
				id = 9;
			} else {
				id = 14;
			}
			map.push(Tile::new(id, atlas));
		}

		let view = View::new(0, 0, view_width, view_height);

		let dirs = vec![1, 0, 4, 5];
		let player = Player::new(dirs, &atlas, 0, 0);

		Map {
			player: player,
			map: map,
			view: view,
			height: height,
			width: width,
		}
	}

	pub fn get(&self, x: i32, y: i32) -> Option<&Tile> {
		let tile = self.map.get(translate(x, y, self.width));
		return tile;
	}

	pub fn set(&mut self, x: i32, y: i32, id: u32) {
		self.map.get_mut(translate(x, y, self.width)).unwrap().tex_id = id;
	}

	pub fn size(&self) -> i32 {
		self.width * self.height
	}

	pub fn track_player(&mut self) {
		if self.player.x > self.width {
			self.player.x = self.width;
		}
		if self.player.y > self.height {
			self.player.y = self.height;
		}

		if self.player.x >= (self.view.width + self.view.x) {
			self.view.x += 1;
		}
		if self.player.x < self.view.x {
			self.view.x -= 1;
		}

		if self.player.y >= (self.view.height + self.view.y) {
			self.view.y += 1;
		}
		if self.player.y < self.view.y {
			self.view.y -= 1;
		}


		if self.view.x < 0 {
			self.view.x = 0;
		}
		if self.view.y < 0 {
			self.view.y = 0;
		}
		if self.view.x > (self.width - self.view.width) {
			self.view.x = self.width - self.view.width;
		}
		if self.view.y > (self.height - self.view.height) {
			self.view.y = self.height - self.view.height;
		}

		//println!("[TRACK][player: {},{}][view: {},{}]", self.player.x, self.player.y, self.view.x, self.view.y);
	}

	pub fn draw(&mut self, mut target: &mut glium::Frame, program: &glium::Program) {
		self.track_player();
		for x in 0..self.view.width {
			for y in 0..self.view.height {
				let scaled_x = rerange(x as f32, 0.0, ((self.view.width as f32) - 1.0), -0.90, 0.90);
				let scaled_y = rerange(y as f32, 0.0, ((self.view.height as f32) - 1.0), -0.90 + 0.0625, 0.90);

				let matrix = [
					[1.0, 0.0, 0.0, 0.0],
					[0.0, 1.0, 0.0, 0.0],
					[0.0, 0.0, 1.0, 0.0],
					[scaled_x, scaled_y, 0.0, 1.0f32],
				];
				let tile = self.map.get(translate(self.view.x + x, self.view.y + y, self.width)).unwrap();
				tile.draw(target, &program, matrix);
			}
		}

		self.player.draw(target, &program, &self.view);
	}
}
