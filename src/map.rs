use glium;

use player::Player;
use entity::Entity;
use tile::{Tile, TileAtlas};
use utils::{rerange, translate};

pub struct View {
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
}

impl View {
	pub fn new(start_x: f32, start_y: f32, width: f32, height: f32) -> View {
		View {
			x: start_x,
			y: start_y,
			width: width,
			height: height,
		}
	}
}

pub struct Map<'a> {
	pub tile_map: Vec<Tile<'a>>,
	pub entity_map: Vec<Entity<'a>>,
	pub player: Player<'a>,
	pub view: View,
	pub height: i32,
	pub width: i32,
}

impl<'a> Map<'a> {
	pub fn new(width: i32, height: i32, view_width: f32, ratio: f32, atlas: &'a TileAtlas) -> Map<'a> {
		let mut tile_map = Vec::with_capacity((width * height) as usize);
		let mut entity_map = Vec::new();

		for index in 0..tile_map.capacity() {
			let x = (index as i32) % width;
			let y = (index as i32) / width;
			let id;
			if (x % 5) == 0 || (y % 5) == 0 {
				id = 10;
			} else {
				id = 9;
			}
			tile_map.push(Tile::new(id, atlas));
		}

		for i in 0..tile_map.len() {
			let x = (i as i32) % width;
			let y = (i as i32) / width;

			if (x % 2) == 0 && (y % 2) == 0 && (x % 5) != 0  && (y % 5) != 0 {
				entity_map.push(Entity::new(x as f32, y as f32, 14, atlas));
			}
		}

		let view = View::new(0.0, 0.0, view_width, (view_width / ratio).floor());

		let dirs = vec![1, 0, 4, 5];
		let player = Player::new(dirs, &atlas, 0.0, 0.0);

		Map {
			tile_map: tile_map,
			entity_map: entity_map,
			player: player,
			view: view,
			height: height,
			width: width,
		}
	}

	pub fn get(&self, x: i32, y: i32) -> Option<&Tile> {
		let tile = self.tile_map.get(translate(x, y, self.width));
		return tile;
	}

	pub fn set(&mut self, x: i32, y: i32, id: u32) {
		self.tile_map.get_mut(translate(x, y, self.width)).unwrap().tex_id = id;
	}

	pub fn size(&self) -> i32 {
		self.width * self.height
	}

	pub fn draw(&mut self, mut target: &mut glium::Frame, program: &glium::Program) {
		for x in 0..(self.view.width as u32) {
			for y in 0..(self.view.height as u32) {
				let x = x as f32;
				let y = y as f32;

				let ui_shim = 0.075;
				let scaled_x = rerange(x, 0.0, self.view.width - 1.0, -1.0, 1.0);
				let scaled_y = rerange(y, 0.0, self.view.height - 1.0, -1.0, 1.0 - ui_shim);

				let tile_width = 1.0 / (self.view.width - 1.0);
				let tile_height = 1.0 / (self.view.height - 1.0);

				let matrix = [
					[1.0 * tile_width, 0.0, 0.0, 0.0],
					[0.0, (1.0 * tile_height) - (ui_shim * (1.0 / (self.view.height * 2.0))), 0.0, 0.0],
					[0.0, 0.0, 1.0, 0.0],
					[scaled_x + tile_width, scaled_y + tile_height + ui_shim, 0.0, 1.0f32],
				];
				let tile = self.tile_map.get(translate((self.view.x + x) as i32, (self.view.y + y) as i32, self.width)).unwrap();
				tile.draw(target, &program, matrix);
			}
		}

		for entity in self.entity_map.iter() {
			let x = entity.x;
			let y = entity.y;

			if x <= self.view.width && y <= self.view.height {
				let ui_shim = 0.075;
				let scaled_x = rerange(x, 0.0, self.view.width - 1.0, -1.0, 1.0);
				let scaled_y = rerange(y, 0.0, self.view.height - 1.0, -1.0, 1.0 - ui_shim);

				let tile_width = 1.0 / (self.view.width - 1.0);
				let tile_height = 1.0 / (self.view.height - 1.0);

				let matrix = [
					[1.0 * tile_width, 0.0, 0.0, 0.0],
					[0.0, (1.0 * tile_height) - (ui_shim * (1.0 / (self.view.height * 2.0))), 0.0, 0.0],
					[0.0, 0.0, 1.0, 0.0],
					[scaled_x + tile_width, scaled_y + tile_height + ui_shim, 0.0, 1.0f32],
				];
				entity.draw(target, &program, matrix);
			}
		}

		self.player.draw(target, &program, &self.view);
	}
}
