#[macro_use]
extern crate glium;
extern crate image;
extern crate glium_text;
extern crate time;

pub mod utils;
pub mod map;
pub mod tile;
pub mod entity;
pub mod player;
pub mod vert;
pub mod game;
pub mod menu;

use glium::{DisplayBuild, Surface};

use menu::Menu;
use game::Game;
use tile::TileAtlas;

#[derive(PartialEq)]
pub enum SceneTrans {
	Quit,
	Menu,
	Game,
}

pub trait Scene<'a> {
    fn handle_input(&mut self, key: Option<glium::glutin::VirtualKeyCode>, state: Option<glium::glutin::ElementState>, mouse_coords: Option<(i32, i32)>, clicked: Option<glium::glutin::MouseButton>, dt: f32) -> SceneTrans;
	fn draw(&mut self, mut target: &mut glium::Frame, program: &glium::Program, text_system: &'a glium_text::TextSystem, font: &'a glium_text::FontTexture);
}

fn main() {
	let mut width = 640;
	let mut height = 480;

	let display = glium::glutin::WindowBuilder::new()
		.with_dimensions(width, height)
		.with_title(format!("TilePaste"))
		.with_vsync()
		.build_glium().unwrap();

	let mut ratio = width as f32 / height as f32;
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
	let mut menu = Menu::new(&display, "assets/main_menu.png", ratio);
	let mut game = Game::new(ratio, &atlas);

	let mut dt = 0.0;
	let mut coords = None;
	let mut game_state = SceneTrans::Menu;

	loop {
		let start_time = time::precise_time_ns();

		let mut key = None;
		let mut key_state = None;
		let mut mouse = None;
		for event in display.poll_events() {
			match event {
				glium::glutin::Event::Closed => { game_state = SceneTrans::Quit; },
				glium::glutin::Event::KeyboardInput(tmp_state, _, tmp_key) => {
					key = tmp_key;
					key_state = Some(tmp_state);
				},
				glium::glutin::Event::MouseMoved(c) => { coords = Some(c); },
				glium::glutin::Event::MouseInput(e, b) => { mouse = Some(b); },
				glium::glutin::Event::Resized(tmp_height, tmp_width) => {
					height = tmp_height;
					width = tmp_width;
					ratio = (height as f32) / (width as f32);
				},
				_ => (),
			}
		}

		if game_state != SceneTrans::Quit {
			let mut target = display.draw();
			target.clear_color(0.0, 0.0, 1.0, 1.0);

			match game_state {
				SceneTrans::Quit => (),
				SceneTrans::Menu => {
					game_state = menu.handle_input(key, key_state, coords, mouse, dt);
					menu.draw(&mut target, &program, &text_system, &font);
				},
				SceneTrans::Game => {
					game_state = game.handle_input(key, key_state, coords, mouse, dt);
					game.draw(&mut target, &program, &text_system, &font);
				},
			}

			target.finish().unwrap();
		}

		let end_time = time::precise_time_ns();
		dt = ((end_time - start_time) as f32 / 1e6) / 60.0;

		if game_state == SceneTrans::Quit {
			return;
		}
	}
}
