use std::fs::File;

use image;
use glium;
use glium::Surface;
use glium_text;

use Scene;
use SceneTrans;
use vert::Vert;
use utils::rerange;

pub struct Menu {
    pub tex: glium::texture::SrgbTexture2d,
    pub buffer: glium::VertexBuffer<Vert>,
    pub buttons: Vec<Button>,
    pub ratio: f32,
}

impl Menu {
    pub fn new(display: &glium::backend::glutin_backend::GlutinFacade, tex_name: &str, ratio: f32) -> Menu {
        let tex_file = File::open(tex_name).unwrap();
        let img = image::load(tex_file, image::PNG).unwrap().to_rgba();
        let dims = img.dimensions();
        let raw_img = glium::texture::RawImage2d::from_raw_rgba_reversed(img.into_raw(), dims);
        let texture = glium::texture::SrgbTexture2d::new(display, raw_img).unwrap();

        let vert1 = Vert { position: [-1.0, -1.0], tex_coords: [ 0.0, 0.0] };
        let vert2 = Vert { position: [-1.0,  1.0], tex_coords: [ 0.0, 1.0] };
        let vert3 = Vert { position: [ 1.0, -1.0], tex_coords: [ 1.0, 0.0] };
        let vert4 = Vert { position: [ 1.0, -1.0], tex_coords: [ 1.0, 0.0] };
        let vert5 = Vert { position: [-1.0,  1.0], tex_coords: [ 0.0, 1.0] };
        let vert6 = Vert { position: [ 1.0,  1.0], tex_coords: [ 1.0, 1.0] };
        let verts = [vert1, vert2, vert3, vert4, vert5, vert6];
        let buffer = glium::VertexBuffer::immutable(display, &verts).unwrap();

        let mut buttons = Vec::new();
        let start_button = Button::new(display, -0.25, -0.4, 0.3, 0.2, String::from("Start"), String::from("assets/button.png"), ratio);
        let quit_button = Button::new(display, 0.25, -0.4, 0.3, 0.2, String::from("Quit"), String::from("assets/button.png"), ratio);
        buttons.push(start_button);
        buttons.push(quit_button);

        Menu {
            tex: texture,
            buffer: buffer,
            buttons: buttons,
            ratio: ratio,
        }
    }
}

impl<'a> Scene<'a> for Menu {
    fn handle_input(&mut self, key: Option<glium::glutin::VirtualKeyCode>, state: Option<glium::glutin::ElementState>, coords: Option<(i32, i32)>, clicked: Option<glium::glutin::MouseButton>, dt: f32) -> SceneTrans {
        if coords.is_some() {
            let coords = coords.unwrap();

            let x = rerange(coords.0 as f32, 0.0, 1280.0, -1.0, 1.0);
            let y = -rerange(coords.1 as f32, 0.0, 960.0, -1.0, 1.0);

            for button in self.buttons.iter() {
                if button.is_hovered(x, y) {
                    if clicked.is_some() && clicked.unwrap() == glium::glutin::MouseButton::Left {
                        button.trigger();
                    }
                }
            }
        }

        if key.is_some() && state.is_some() && state.unwrap() == glium::glutin::ElementState::Pressed {
    		let key = key.unwrap();
    		match key {
                glium::glutin::VirtualKeyCode::Return => { return SceneTrans::Game; },
    			glium::glutin::VirtualKeyCode::Q => { return SceneTrans::Quit; }
    			_ => { return SceneTrans::Menu; },
    		}
    	}
    	return SceneTrans::Menu;
    }

    fn draw(&mut self, mut target: &mut glium::Frame, program: &glium::Program, text_system: &glium_text::TextSystem, font: &glium_text::FontTexture) {
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let uniform = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ],
            tex: self.tex.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
        };

        target.draw(&self.buffer, &indices, program, &uniform, &Default::default()).unwrap();
        for button in self.buttons.iter() {
            button.draw(&mut target, program, text_system, font);
        }
    }
}

pub struct Button {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    text: String,
    ratio: f32,
    tex: glium::texture::SrgbTexture2d,
    buffer: glium::VertexBuffer<Vert>,
}

impl Button {
    pub fn new(display: &glium::backend::glutin_backend::GlutinFacade, x: f32, y: f32, width: f32, height: f32, text: String, tex_name: String, ratio: f32) -> Button {
        let tex_file = File::open(tex_name).unwrap();
        let img = image::load(tex_file, image::PNG).unwrap().to_rgba();
        let dims = img.dimensions();
        let raw_img = glium::texture::RawImage2d::from_raw_rgba_reversed(img.into_raw(), dims);
        let texture = glium::texture::SrgbTexture2d::new(display, raw_img).unwrap();

        let vert1 = Vert { position: [-1.0, -1.0], tex_coords: [ 0.0, 0.0] };
        let vert2 = Vert { position: [-1.0,  1.0], tex_coords: [ 0.0, 1.0] };
        let vert3 = Vert { position: [ 1.0, -1.0], tex_coords: [ 1.0, 0.0] };
        let vert4 = Vert { position: [ 1.0, -1.0], tex_coords: [ 1.0, 0.0] };
        let vert5 = Vert { position: [-1.0,  1.0], tex_coords: [ 0.0, 1.0] };
        let vert6 = Vert { position: [ 1.0,  1.0], tex_coords: [ 1.0, 1.0] };
        let verts = [vert1, vert2, vert3, vert4, vert5, vert6];
        let buffer = glium::VertexBuffer::immutable(display, &verts).unwrap();

        Button {
            x: x,
            y: y,
            width: width,
            height: height,
            text: text,
            buffer: buffer,
            ratio: ratio,
            tex: texture,
        }
    }

    fn is_hovered(&self, x: f32, y: f32) -> bool {
        (x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height)
    }

    fn trigger(&self) {
        println!("{}: triggered!, ({},{})", self.text, self.x, self.y);
    }

    fn draw(&self, mut target: &mut glium::Frame, program: &glium::Program, text_system: &glium_text::TextSystem, font: &glium_text::FontTexture) {
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let uniform = uniform! {
            matrix: [
                [1.0 * 0.1, 0.0, 0.0, 0.0],
                [0.0, 1.0 * 0.1, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [self.x, self.y, 0.0, 1.0f32],
            ],
            tex: self.tex.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
        };

        let text_matrix = [
            [0.05 / self.ratio, 0.0, 0.0, 0.0],
            [0.0, 0.05, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [self.x - (self.width / 2.0), self.y - (self.height / 4.0), 0.0, 1.0],
        ];

        target.draw(&self.buffer, &indices, program, &uniform, &Default::default()).unwrap();

        let text = glium_text::TextDisplay::new(text_system, font, self.text.as_str());
        glium_text::draw(&text, text_system, target, text_matrix, (1.0, 1.0, 0.0, 1.0));
    }
}
