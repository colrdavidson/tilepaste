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
			gl_Position = matrix * vec4(position, 0.0, 1.0);
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
				glium::glutin::Event::KeyboardInput(state, _, key) => if game.handle_input(key, state) { return; },
				_ => (),
			}
		}
	}
}
