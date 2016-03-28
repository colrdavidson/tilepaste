use glium;

use tile::{Tile, TileAtlas};

pub struct Player<'a> {
	pub x: u32,
	pub y: u32,
	pub tile: Tile<'a>, //location & texture id
}

impl<'a> Player<'a> {
	pub fn new(id: u32, atlas: &'a TileAtlas, x: u32, y: u32, display: &glium::backend::glutin_backend::GlutinFacade) -> Player<'a> {
		Player {
			x: x,
			y: y,
			tile: Tile::new(id, atlas, display),
		}
	}

	pub fn move_to(&mut self, x: i32, y: i32) {
		self.x = ((self.x as i32) + x) as u32;
		self.y = ((self.y as i32) + y) as u32;
	}

	pub fn down(&mut self) { println!("player down!"); }

	pub fn up(&mut self) { println!("player up!"); }

	pub fn left(&mut self) { println!("player left!"); }

	pub fn right(&mut self) { println!("player right!"); }
}
