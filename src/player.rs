use glium;

use tile::{Tile, TileAtlas};
use map::View;
use utils::rerange;

pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

pub struct Player<'a> {
	pub x: i32,
	pub y: i32,
	pub dir: Direction,
	pub up: Tile<'a>,
	pub down: Tile<'a>,
	pub left: Tile<'a>,
	pub right: Tile<'a>,
}

impl<'a> Player<'a> {
	pub fn new(dirs: Vec<u32>, atlas: &'a TileAtlas, x: i32, y: i32) -> Player<'a> {
		Player {
			x: x,
			y: y,
			dir: Direction::Down,
			up: Tile::new(dirs[0], atlas),
			down: Tile::new(dirs[1], atlas),
			left: Tile::new(dirs[2], atlas),
			right: Tile::new(dirs[3], atlas),
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
		self.dir = Direction::Down;
		self.move_to(0, -1);
	}

	pub fn up(&mut self) {
		self.dir = Direction::Up;
		self.move_to(0, 1);
	}

	pub fn left(&mut self) {
		self.dir = Direction::Left;
		self.move_to(-1, 0);
	}

	pub fn right(&mut self) {
		self.dir = Direction::Right;
		self.move_to(1, 0);
	}

	pub fn draw(&mut self, mut target: &mut glium::Frame, program: &glium::Program, view: &View) {
		let x;
		if self.x > (view.width - 1) {
			x = view.width - 1;
		} else {
			x = self.x;
		}
		let y;
		if self.y > (view.height - 1) {
			y = view.height - 1;
		} else {
			y = self.y;
		}
		let scaled_x = rerange(x as f32, 0.0, ((view.width as f32) - 1.0), -0.90, 0.90);
		let scaled_y = rerange(y as f32, 0.0, ((view.height as f32) - 1.0), -0.90 + 0.0625, 0.90);
		let matrix = [
				[1.0, 0.0, 0.0, 0.0],
				[0.0, 1.0, 0.0, 0.0],
				[0.0, 0.0, 1.0, 0.0],
				[scaled_x, scaled_y, 0.0, 1.0f32],
		];

		let tile;
		match self.dir {
			Direction::Up => { tile = &self.up; },
			Direction::Down => { tile = &self.down; },
			Direction::Left => { tile = &self.left; },
			Direction::Right => { tile = &self.right; },
		}
		tile.draw(&mut target, program, matrix);
	}
}
