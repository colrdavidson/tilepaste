use glium;
use glium_text;
use glium::Surface;

use Scene;
use SceneTrans;
use player::Player;
use map::Map;
use tile::TileAtlas;
use utils::V2;

pub struct Game<'a> {
    pub player: Player<'a>,
    pub map: Map<'a>,
    pub score: u32,
    pub ratio: f32,
}

impl<'a> Game<'a> {
    pub fn new(ratio: f32, atlas: &'a TileAtlas) -> Game {
        let map = Map::new(101, 101, 20.0, ratio, &atlas);

        let dirs = vec![1, 0, 4, 5];
        let player = Player::new(dirs, &atlas, V2::new(0.0, 0.0));
    	let score = 9999;

        Game {
            player: player,
            map: map,
            score: score,
            ratio: ratio,
        }
    }
}

impl<'a> Scene<'a> for Game<'a> {
    fn handle_input(&mut self, key: Option<glium::glutin::VirtualKeyCode>, state: Option<glium::glutin::ElementState>, coords: Option<(i32, i32)>, clicked: Option<glium::glutin::MouseButton>, dt: f32) -> SceneTrans {
    	if key.is_some() && state.is_some() && ((state.unwrap() == glium::glutin::ElementState::Pressed) || (state.unwrap() == glium::glutin::ElementState::Released)) {
    		let key = key.unwrap();
    		match key {
    			glium::glutin::VirtualKeyCode::W => { self.player.up(dt); return SceneTrans::Game; },
    			glium::glutin::VirtualKeyCode::S => { self.player.down(dt); return SceneTrans::Game; },
    			glium::glutin::VirtualKeyCode::A => { self.player.left(dt); return SceneTrans::Game; },
    			glium::glutin::VirtualKeyCode::D => { self.player.right(dt); return SceneTrans::Game; },
                glium::glutin::VirtualKeyCode::Escape => { return SceneTrans::Menu; },
    			glium::glutin::VirtualKeyCode::Q => { return SceneTrans::Quit; }
    			_ => { self.player.move_to(0.0, 0.0, dt); return SceneTrans::Game; },
    		}
    	} else {
            self.player.move_to(0.0, 0.0, dt);
        }
    	return SceneTrans::Game;
    }

    fn draw(&mut self, mut target: &mut glium::Frame, program: &glium::Program, text_system: &glium_text::TextSystem, font: &glium_text::FontTexture) {
        self.map.draw(&mut target, &program);
        self.player.draw(target, &program, &self.map.view);

    	let score_matrix = [
    		[0.05 / self.ratio, 0.0, 0.0, 0.0],
    		[0.0, 0.05, 0.0, 0.0],
    		[0.0, 0.0, 1.0, 0.0],
    		[0.65, -0.99, 0.0, 1.0],
    	];

    	let title_matrix = [
    		[0.05 / self.ratio, 0.0, 0.0, 0.0],
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
