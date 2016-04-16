use std::f32;

use glium;

use tile::{Tile, TileAtlas};
use map::View;
use utils::rerange;
use utils::V2;
use keyboard;

pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

pub struct Player<'a> {
	pub pos: V2<f32>,
	pub vel: V2<f32>,
	pub dir: Direction,
	pub up: Tile<'a>,
	pub down: Tile<'a>,
	pub left: Tile<'a>,
	pub right: Tile<'a>,
}

impl<'a> Player<'a> {
	pub fn new(dirs: Vec<u32>, atlas: &'a TileAtlas, pos: V2<f32>) -> Player<'a> {
		Player {
			pos: pos,
			vel: V2::new(0.0, 0.0),
			dir: Direction::Down,
			up: Tile::new(dirs[0], atlas),
			down: Tile::new(dirs[1], atlas),
			left: Tile::new(dirs[2], atlas),
			right: Tile::new(dirs[3], atlas),
		}
	}

	pub fn move_to(&mut self, x: f32, y: f32, dt: f32) {
        let friction = -1.0;

		let x = friction * self.vel.x + x;
		let y = friction * self.vel.y + y;

		self.pos.x = (0.5 * x * dt * dt) + self.vel.x * dt + self.pos.x;
		self.pos.y = (0.5 * y * dt * dt) + self.vel.y * dt + self.pos.y;
		self.vel.x = (x * dt) + self.vel.x;
		self.vel.y = (y * dt) + self.vel.y;

		if !(self.pos.x > 0.0 && self.pos.y > 0.0) {
			if self.pos.x < 0.0 {
				self.pos.x = 0.0;
				self.vel.x = 0.0;
			}
			if self.pos.y < 0.0 {
				self.pos.y = 0.0;
				self.vel.y = 0.0;
			}
		}
	}

    pub fn handle_input(&mut self, keys: Vec<&keyboard::Action>, t: f32) {
        let mut mx = 0.0;
        let mut my = 0.0;
        for key in keys {
            match *key {
                keyboard::Action::Up => { my += 1.0; },
                keyboard::Action::Down => { my += -1.0; },
                keyboard::Action::Left => { mx += -1.0; },
                keyboard::Action::Right => { mx += 1.0; },
                _ => { },
            }
        }

        if mx > 1.0 { mx = 1.0; }
        if mx < -1.0 { mx = -1.0; }
        if my > 1.0 { my = 1.0; }
        if my < -1.0 { my = -1.0; }

        let diag = (f32::consts::PI / 4.0).sin();
        if mx == 1.0 && my == 1.0 {
            mx = diag;
            my = diag;
        }

        if mx == 1.0 && my == -1.0 {
            mx = diag;
            my = -diag;
        }

        if mx == -1.0 && my == 1.0 {
            mx = -diag;
            my = diag;
        }

        if mx == -1.0 && my == -1.0 {
            mx = -diag;
            my = -diag;
        }

        if mx == 1.0 && my == 0.0 {
            self.dir = Direction::Right;
        }
        if mx == -1.0 && my == 0.0 {
            self.dir = Direction::Left;
        }
        if mx == 0.0 && my == -1.0 {
            self.dir = Direction::Down;
        }
        if mx == 0.0 && my == 1.0 {
            self.dir = Direction::Up;
        }

        self.move_to(mx, my, t);
    }

	pub fn down(&mut self, t: f32) {
		self.dir = Direction::Down;
		self.move_to(0.0, -1.0, t);
	}

	pub fn up(&mut self, t: f32) {
		self.dir = Direction::Up;
		self.move_to(0.0, 1.0, t);
	}

	pub fn left(&mut self, t: f32) {
		self.dir = Direction::Left;
		self.move_to(-1.0, 0.0, t);
	}

	pub fn right(&mut self, t: f32) {
		self.dir = Direction::Right;
		self.move_to(1.0, 0.0, t);
	}

	pub fn draw(&mut self, mut target: &mut glium::Frame, program: &glium::Program, view: &View) {

		let ui_shim = 0.075;
		let scaled_x = rerange(self.pos.x, 0.0, view.width - 1.0, -1.0, 1.0);
		let scaled_y = rerange(self.pos.y, 0.0, view.height - 1.0, -1.0, 1.0 - ui_shim);

		let tile_width = 1.0 / (view.width - 1.0);
		let tile_height = 1.0 / (view.height - 1.0);

		let matrix = [
			[1.0 * tile_width, 0.0, 0.0, 0.0],
			[0.0, (1.0 * tile_height) - (ui_shim * (1.0 / (view.height * 2.0))), 0.0, 0.0],
			[0.0, 0.0, 1.0, 0.0],
			[scaled_x + tile_width, scaled_y + tile_height + ui_shim, 0.0, 1.0f32],
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
