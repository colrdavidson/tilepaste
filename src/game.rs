use glium;
use glium_text;
use glium::Surface;

use map::Map;
use tile::TileAtlas;

pub struct Game<'a> {
    pub map: Map<'a>,
    pub score: u32,
}

impl<'a> Game<'a> {
    pub fn new(ratio: f32, atlas: &'a TileAtlas) -> Game {
        let map = Map::new(101, 101, 20.0, ratio, &atlas);
    	let score = 9999;

        Game {
            map: map,
            score: score,
        }
    }

    pub fn handle_input(&mut self, key: Option<glium::glutin::VirtualKeyCode>, state: glium::glutin::ElementState, dt: f32) -> bool {
    	if key.is_some() && state == glium::glutin::ElementState::Pressed {
    		let key = key.unwrap();
    		match key {
    			glium::glutin::VirtualKeyCode::W => { self.map.player.up(dt); return false; },
    			glium::glutin::VirtualKeyCode::S => { self.map.player.down(dt); return false; },
    			glium::glutin::VirtualKeyCode::A => { self.map.player.left(dt); return false; },
    			glium::glutin::VirtualKeyCode::D => { self.map.player.right(dt); return false; },
    			glium::glutin::VirtualKeyCode::Space => { println!("SPACE"); return false; },
    			glium::glutin::VirtualKeyCode::Q => { return true; }
    			_ => { return false; },
    		}
    	}
    	false
    }

    pub fn draw(&mut self, mut target: &mut glium::Frame, program: &glium::Program, text_system: &glium_text::TextSystem, font: &glium_text::FontTexture, ratio: f32) {
        self.map.draw(&mut target, &program);

    	let score_matrix = [
    		[0.05 / ratio, 0.0, 0.0, 0.0],
    		[0.0, 0.05, 0.0, 0.0],
    		[0.0, 0.0, 1.0, 0.0],
    		[0.65, -0.99, 0.0, 1.0],
    	];

    	let title_matrix = [
    		[0.05 / ratio, 0.0, 0.0, 0.0],
    		[0.0, 0.05, 0.0, 0.0],
    		[0.0, 0.0, 1.0, 0.0],
    		[-1.0, -0.99, 0.0, 1.0],
    	];

        let title_text = glium_text::TextDisplay::new(text_system, font, "TilePaste");
        let score_text = glium_text::TextDisplay::new(text_system, font, format!("score: {}", self.score).as_str());
        glium_text::draw(&score_text, text_system, target, score_matrix, (1.0, 1.0, 0.0, 1.0));
        glium_text::draw(&title_text, text_system, target, title_matrix, (1.0, 1.0, 0.0, 1.0));
    }
}
