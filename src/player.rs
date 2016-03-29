use glium;

use tile::{Tile, TileAtlas};
use map::Map;
use utils::rerange;

pub struct Player<'a> {
	pub x: i32,
	pub y: i32,
	pub tile: Tile<'a>, //location & texture id
}

impl<'a> Player<'a> {
	pub fn new(id: u32, atlas: &'a TileAtlas, x: i32, y: i32) -> Player<'a> {
		Player {
			x: x,
			y: y,
			tile: Tile::new(id, atlas),
		}
	}

	pub fn move_to(&mut self, x: i32, y: i32) {
		self.x += x;
		self.y += y;

		if !(self.x > 0 && self.y > 0) {
			if self.x < 0 {
				self.x = 0;
			}
			if self.y < 0 {
				self.y = 0;
			}
		}
	}

	pub fn down(&mut self) {
		self.move_to(0, -1);
	}

	pub fn up(&mut self) {
		self.move_to(0, 1);
	}

	pub fn left(&mut self) {
		self.move_to(-1, 0);
	}

	pub fn right(&mut self) {
		self.move_to(1, 0);
	}

	pub fn draw(&self, mut target: &mut glium::Frame, program: &glium::Program, map: &Map, ratio: f32) {
		let scaled_x = rerange((self.x as f32) / ratio, 0.0, ((map.view.width as f32) - 1.0), -0.90, 0.90);
		let scaled_y = rerange(self.y as f32, 0.0, ((map.view.height as f32) - 1.0), -0.90 + 0.0625, 0.90);
		let matrix = [
				[1.0 / ratio, 0.0, 0.0, 0.0],
				[0.0, 1.0, 0.0, 0.0],
				[0.0, 0.0, 1.0, 0.0],
				[scaled_x, scaled_y, 0.0, 1.0f32],
		];
		self.tile.draw(&mut target, program, matrix);
	}
}
