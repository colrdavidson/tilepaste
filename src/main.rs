#[macro_use]
extern crate glium;
extern crate image;

pub mod map;
pub mod player;

use glium::{DisplayBuild, Surface};
use std::io::Cursor;

use map::{Map, View};
use map::rerange;
use player::Player;

#[derive(Copy, Clone)]
struct Vert {
	position: [f32; 2],
	tex_coords: [f32; 2],
}

implement_vertex!(Vert, position, tex_coords);

fn atlas_verts(entry: usize) -> Vec<Vert> {
	let num_entries = 16;

	let scalar = 1.0 / ((num_entries as f32) / 4.0);

	let base_x = entry % (num_entries / 4);
	let base_y = entry / (num_entries / 4);
	let base_x = (base_x as f32) * scalar;
	let base_y = (base_y as f32) * scalar;

	let bottom_left =  [base_x, base_y];
	let bottom_right = [base_x + scalar, base_y];
	let top_left = 	   [base_x, base_y + scalar];
	let top_right =	   [base_x + scalar, base_y + scalar];

	let vert1 = Vert { position: [-0.5, -0.5], tex_coords: bottom_left };
	let vert2 = Vert { position: [-0.5,  0.5], tex_coords: top_left };
	let vert3 = Vert { position: [ 0.5, -0.5], tex_coords: bottom_right };
	let vert4 = Vert { position: [ 0.5, -0.5], tex_coords: bottom_right };
	let vert5 = Vert { position: [-0.5,  0.5], tex_coords: top_left };
	let vert6 = Vert { position: [ 0.5,  0.5], tex_coords: top_right };
	vec![vert1, vert2, vert3, vert4, vert5, vert6]
}

fn handle_input(key: Option<glium::glutin::VirtualKeyCode>, state: glium::glutin::ElementState, mut view: &mut View, player: &mut Player) {
	if key.is_some() && state == glium::glutin::ElementState::Pressed {
		let key = key.unwrap();
		match key {
			glium::glutin::VirtualKeyCode::W => { player.up(&mut view); },
			glium::glutin::VirtualKeyCode::S => { player.down(&mut view); },
			glium::glutin::VirtualKeyCode::A => { player.left(&mut view); },
			glium::glutin::VirtualKeyCode::D => { player.right(&mut view); },
			_ => (),
		}
	}
}

fn main() {
	let display = glium::glutin::WindowBuilder::new()
		.with_dimensions(640, 480)
		.with_title(format!("TilePaste"))
		.with_vsync()
		.build_glium().unwrap();

	let image = image::load(Cursor::new(&include_bytes!("../assets/atlas.png")[..]), image::PNG).unwrap().to_rgba();
	let image_dimensions = image.dimensions();
	let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
	let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

	let mut map = Map::new(101, 101);

	let bee = atlas_verts(13);
	let apple = atlas_verts(12);
	let hat = atlas_verts(8);
	let grass = atlas_verts(9);
	let stone = atlas_verts(10);
	let obox = atlas_verts(15);
	let tree = atlas_verts(14);

	let bee_buffer = glium::VertexBuffer::immutable(&display, &bee).unwrap();
	let apple_buffer = glium::VertexBuffer::immutable(&display, &apple).unwrap();
	let hat_buffer = glium::VertexBuffer::immutable(&display, &hat).unwrap();
	let grass_buffer = glium::VertexBuffer::immutable(&display, &grass).unwrap();
	let box_buffer = glium::VertexBuffer::immutable(&display, &obox).unwrap();
	let stone_buffer = glium::VertexBuffer::immutable(&display, &stone).unwrap();
	let tree_buffer = glium::VertexBuffer::immutable(&display, &tree).unwrap();

	let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

	let vert_shader_src = r#"
		#version 140

		in vec2 position;
		in vec2 tex_coords;
		out vec2 v_tex_coords;

		uniform mat4 matrix;
		void main() {
			v_tex_coords = tex_coords;
			gl_Position = matrix * vec4(position * 0.1, 0.0, 1.0);
		}
	"#;

	let frag_shader_src = r#"
		#version 140

		in vec2 v_tex_coords;
		out vec4 color;

		uniform sampler2D tex;
		void main() {
			color = texture(tex, v_tex_coords);
			if (color.a == 0.0) { discard; }
		}
	"#;

	let program = glium::Program::from_source(&display, vert_shader_src, frag_shader_src, None).unwrap();

	let mut view = View::new(0, 0, 20, 20, map.width as u32, map.height as u32);
	let mut player = Player::new(map.width as u32, map.height as u32, 0, 0, 13);

	loop {
		let mut target = display.draw();
		target.clear_color(0.0, 0.0, 1.0, 1.0);

		if map.get(player.x, player.y).unwrap().tex_id == 2 {
			map.set(player.x, player.y, 4);
		}

		for x in 0..view.width {
			for y in 0..view.height {
				let tile = map.uniform(x, y, x + view.x, y + view.y, view.width as u32, view.height as u32);
				let tile_uniforms = uniform! {
					matrix: tile.0,
					tex: texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
				};

				let buffer;
				match tile.1 {
					0 => { buffer = &box_buffer; },
					1 => { buffer = &grass_buffer; },
					2 => { buffer = &tree_buffer; },
					3 => { buffer = &stone_buffer; },
					4 => { buffer = &apple_buffer; },
					_ => { buffer = &box_buffer; },
				}
				target.draw(buffer, &indices, &program, &tile_uniforms, &Default::default()).unwrap();
			}
		}

		let p_x = rerange(player.tile.x as f32, 0.0, ((view.width as f32) - 1.0), -0.90, 0.90);
		let p_y = rerange(player.tile.y as f32, 0.0, ((view.height as f32) - 1.0), -0.90, 0.90);

		let player_uniform = uniform! {
			matrix: [
				[1.0, 0.0, 0.0, 0.0],
				[0.0, 1.0, 0.0, 0.0],
				[0.0, 0.0, 1.0, 0.0],
				[p_x, p_y, 0.0, 1.0f32],
			],
			tex: texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
		};
		target.draw(&bee_buffer, &indices, &program, &player_uniform, &Default::default()).unwrap();

		target.finish().unwrap();

		for event in display.poll_events() {
			match event {
				glium::glutin::Event::Closed => return,
				glium::glutin::Event::KeyboardInput(state, _, key) => handle_input(key, state, &mut view, &mut player),
				_ => (),
			}
		}
	}
}
