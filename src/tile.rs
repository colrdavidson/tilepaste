use std::io::Cursor;
use std::fmt;

use image;
use glium;
use glium::Surface;

use vert::Vert;

pub struct Tile<'a> {
	pub tex_id: u32,
    pub atlas: &'a TileAtlas,
}

impl<'a> fmt::Debug for Tile<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tile: {}", self.tex_id)
    }
}

impl<'a> Tile<'a> {
	pub fn new(id: u32, atlas: &'a TileAtlas, display: &glium::backend::glutin_backend::GlutinFacade) -> Tile<'a> {
		Tile {
			tex_id: id,
            atlas: atlas,
		}
	}

    pub fn draw(&self, mut target: &mut glium::Frame, program: &glium::Program, matrix: [[f32; 4]; 4]) {
        let buffer = self.atlas.tex_verts.get(self.tex_id as usize).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let tile_uniform = uniform! {
            matrix: matrix,
            tex: self.atlas.texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
        };

        target.draw(buffer, &indices, program, &tile_uniform, &Default::default()).unwrap();
    }
}

pub struct TileAtlas {
    pub texture: glium::texture::SrgbTexture2d,
    pub tex_verts: Vec<glium::VertexBuffer<Vert>>,
    pub img_width: u32,
    pub img_height: u32,
    pub tile_width: u32,
    pub tile_height: u32,
    pub num_entries: u32,
}

impl TileAtlas {
    pub fn new(display: &glium::backend::glutin_backend::GlutinFacade, tile_width: u32, tile_height: u32) -> TileAtlas {
        let img = image::load(Cursor::new(&include_bytes!("../assets/atlas.png")[..]), image::PNG).unwrap().to_rgba();
        let dims = img.dimensions();
        let raw_img = glium::texture::RawImage2d::from_raw_rgba_reversed(img.into_raw(), dims);
        let texture = glium::texture::SrgbTexture2d::new(display, raw_img).unwrap();

        let img_width = dims.0;
        let img_height = dims.1;

        let x_entries = img_width / tile_width;
        let y_entries = img_height / tile_height;
        let num_entries = (x_entries * y_entries) as usize;

        let mut tex_verts = Vec::with_capacity(num_entries);

        for i in 0..num_entries {
			let buffer = glium::VertexBuffer::immutable(display, atlas_verts(i, num_entries).as_slice()).unwrap();
            tex_verts.push(buffer);
        }

        TileAtlas {
            texture: texture,
            tex_verts: tex_verts,
            img_width: img_width,
            img_height: img_height,
            tile_width: tile_width,
            tile_height: tile_height,
            num_entries: num_entries as u32,
        }
    }
}

fn atlas_verts(entry: usize, sheet_entries: usize) -> Vec<Vert> {
    let num_entries = sheet_entries;
    let col_num = (num_entries as f32).sqrt();
    let row_num = (num_entries as f32).sqrt();

    let scalar = 1.0 / ((num_entries as f32) / col_num);

    let base_x = entry % (num_entries / (col_num as usize));
    let base_y = entry / (num_entries / (row_num as usize));
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
