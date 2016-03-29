use glium;

use tile::{Tile, TileAtlas};
use utils::{rerange, translate};

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

	pub fn down(&mut self) { println!("View down!"); }

	pub fn up(&mut self) { println!("View down!"); }

    pub fn left(&mut self) { println!("View left!"); }

    pub fn right(&mut self) { println!("View right!"); }
}

pub struct Map<'a> {
	pub map: Vec<Tile<'a>>,
	pub view: View,
	pub height: u32,
	pub width: u32,
}

impl<'a> Map<'a> {
	pub fn new(width: u32, height: u32, view_width: u32, view_height: u32, atlas: &'a TileAtlas) -> Map<'a> {
		let mut map = Vec::with_capacity((width * height) as usize);

		for index in 0..map.capacity() {
			let x = (index as u32) % width;
			let y = (index as u32) / width;
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

		Map {
			map: map,
			view: view,
			height: height,
			width: width,
		}
	}

	pub fn uniform(&self, x: u32, y: u32, ratio: f32) -> [[f32; 4]; 4] {
		let scaled_x = rerange((x as f32) / ratio, 0.0, ((self.view.width as f32) - 1.0), -0.90, 0.90);
		let scaled_y = rerange(y as f32, 0.0, ((self.view.height as f32) - 1.0), -0.90 + 0.0625, 0.90);
		//println!("(x,y): {},{}", scaled_x, scaled_y);

		[
			[1.0 / ratio, 0.0, 0.0, 0.0],
			[0.0, 1.0, 0.0, 0.0],
			[0.0, 0.0, 1.0, 0.0],
			[scaled_x, scaled_y, 0.0, 1.0f32],
		]
	}

	pub fn get(&self, x: u32, y: u32) -> Option<&Tile> {
		let tile = self.map.get(translate(x, y, self.width));
		return tile;
	}

	pub fn set(&mut self, x: u32, y: u32, id: u32) {
		self.map.get_mut(translate(x, y, self.width)).unwrap().tex_id = id;
	}

	pub fn size(&self) -> u32 {
		self.width * self.height
	}

	pub fn draw(&mut self, mut target: &mut glium::Frame, program: &glium::Program, ratio: f32) {
		for x in 0..self.view.width {
			for y in 0..self.view.height {
				let tile = self.map.get(translate(x, y, self.view.width)).unwrap();
				let matrix = self.uniform(x, y, ratio);

				tile.draw(target, &program, matrix);
			}
		}
	}
}
