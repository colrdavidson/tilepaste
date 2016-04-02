#[macro_use]
extern crate glium;
extern crate image;
extern crate glium_text;

pub mod utils;
pub mod map;
pub mod tile;
pub mod player;
pub mod vert;
pub mod game;

use glium::{DisplayBuild, Surface};

use game::Game;
use tile::TileAtlas;

fn handle_input(key: Option<glium::glutin::VirtualKeyCode>, state: glium::glutin::ElementState, game: &mut Game) -> bool {
	if key.is_some() && state == glium::glutin::ElementState::Pressed {
		let key = key.unwrap();
		match key {
			glium::glutin::VirtualKeyCode::W => { game.map.player.up(); return false; },
			glium::glutin::VirtualKeyCode::S => { game.map.player.down(); return false; },
			glium::glutin::VirtualKeyCode::A => { game.map.player.left(); return false; },
			glium::glutin::VirtualKeyCode::D => { game.map.player.right(); return false; },
			glium::glutin::VirtualKeyCode::Space => { println!("SPACE"); return false; },
			glium::glutin::VirtualKeyCode::Q => { return true; }
			_ => { return false; },
		}
	}
	false
}

fn main() {
	let width = 640;
	let height = 480;

	let display = glium::glutin::WindowBuilder::new()
		.with_dimensions(width, height)
		.with_title(format!("TilePaste"))
		.with_vsync()
		.build_glium().unwrap();

	let ratio = width as f32 / height as f32;
	let atlas = TileAtlas::new(&display, 16, 16);

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
	let text_system = glium_text::TextSystem::new(&display);
	let font_file = std::fs::File::open(&std::path::Path::new("assets/ubuntu.ttf")).unwrap();
	let font = glium_text::FontTexture::new(&display, font_file, 24).unwrap();

	let mut game = Game::new(&atlas);

	loop {
		let mut target = display.draw();
		target.clear_color(0.0, 0.0, 1.0, 1.0);

		game.draw(&mut target, &program, &text_system, &font, ratio);

		target.finish().unwrap();

		for event in display.poll_events() {
			match event {
				glium::glutin::Event::Closed => return,
				glium::glutin::Event::KeyboardInput(state, _, key) => if handle_input(key, state, &mut game) { return; },
				_ => (),
			}
		}
	}
}
