use glium;
use glium::Surface;

use tile::TileAtlas;

pub struct Entity<'a> {
    pub x: f32,
    pub y: f32,
    pub tex_id: u32,
    pub atlas: &'a TileAtlas,
}

impl<'a> Entity<'a> {
    pub fn new(x: f32, y: f32, id: u32, atlas: &'a TileAtlas) -> Entity<'a> {
        Entity {
            x: x,
            y: y,
            tex_id: id,
            atlas: atlas,
        }
    }

    pub fn draw(&self, mut target: &mut glium::Frame, program: &glium::Program, matrix: [[f32; 4]; 4]) {
        let buffer = self.atlas.tex_verts.get(self.tex_id as usize).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let entity_uniform = uniform! {
            matrix: matrix,
            tex: self.atlas.texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
        };

        target.draw(buffer, &indices, program, &entity_uniform, &Default::default()).unwrap();
    }
}
