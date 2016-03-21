#[macro_use]
extern crate glium;
extern crate image;

pub mod map;

use glium::{DisplayBuild, Surface};
use std::io::Cursor;

use map::Map;

#[derive(Copy, Clone)]
struct Vert {
	position: [f32; 2],
	tex_coords: [f32; 2],
}

implement_vertex!(Vert, position, tex_coords);

fn atlas_verts(entry: usize) -> Vec<Vert> {
	let num_entries = 4;

	let scalar = 1.0 / ((num_entries as f32) / 2.0);

	let base_x = entry % (num_entries / 2);
	let base_y = entry / (num_entries / 2);
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

fn main() {
	let display = glium::glutin::WindowBuilder::new()
		.with_dimensions(640, 480)
		.with_title(format!("TilePaste"))
		.build_glium().unwrap();

	let image = image::load(Cursor::new(&include_bytes!("../assets/atlas.png")[..]), image::PNG).unwrap().to_rgba();
	let image_dimensions = image.dimensions();
	let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
	let texture = glium::texture::Texture2d::new(&display, image).unwrap();

	let map = Map::new(10, 10);

	let grass = atlas_verts(1);
	let grass_buffer = glium::VertexBuffer::new(&display, &grass).unwrap();
	let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

	let vert_shader_src = r#"
		#version 140

		in vec2 position;
		in vec2 tex_coords;
		out vec2 v_tex_coords;

		uniform mat4 matrix;
		void main() {
			v_tex_coords = tex_coords;
			gl_Position = matrix * vec4(position * 0.225, 0.0, 1.0);
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

	loop {
		let mut target = display.draw();
		target.clear_color(0.0, 0.0, 1.0, 1.0);

		for idx in 0..map.size() {
			let x = idx % map.width;
			let y = idx / map.width;
			let matrix = map.uniform(x, y);
			let grass_uniforms = uniform! {
				matrix: matrix,
				tex: texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
			};

			target.draw(&grass_buffer, &indices, &program, &grass_uniforms, &Default::default()).unwrap();
		}

		target.finish().unwrap();

		for event in display.poll_events() {
			match event {
				glium::glutin::Event::Closed => return,
				_ => (),
			}
		}
	}
}
