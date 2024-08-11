use bevy_math::UVec2;
use pixels::wgpu::Color;
use crate::generate_seed;
use rand::Rng;

// FIXME: legacy
use crate::components::particle::Particle;

use crate::components::tile::Tile;

pub struct SimGrid {
    pub width: usize,
    pub height: usize,
    pub pixel_size: usize,
    pub x: u8,
    pub y: u8,
    pub pixels: Vec<Particle>,
    pub tiles: Vec<Tile>,
    pub needsReRender: bool,
}

impl SimGrid {
    pub fn new(width: usize, height: usize, pixel_size: usize) -> Self {
        assert!(width != 0 && height != 0);
        Self {
            x: 0,
            y: 0,
            width,
            height,
            pixel_size,
            pixels: vec![Particle::default(); width * height],
            tiles: vec![Tile::default()],
            needsReRender: true,
        }
    }

    pub fn render_screen(&mut self, screen: &mut [u8]) {
        let mut temp_pixel_colours_from_tiles: Vec<[u8; 4]> = vec![];
        // Render tiles into screen
        for tile in self.tiles.iter() {
            for p in tile.pixels.iter() {
                temp_pixel_colours_from_tiles.push(p.colour_rgba);
            }
        }
        println!("temp_pixels_from_tiles: {}", temp_pixel_colours_from_tiles.len());

        let res: usize = 32;
        let width_res: i32 = res as i32;
        let height_res: i32 = res as i32;
        // let tile_res = width_res * height_res;
        let width_res_fit: i32 = (self.width / res) as i32;
        println!("width_res_fit: {}", width_res_fit);
        let height_res_fit: i32 = (self.height / res) as i32;
        
        for (tile_i, tile) in self.tiles.iter().enumerate() {
            println!("tile_i: {}", tile_i);
            let mut calculated_x = 0;
            let mut calculated_y = 0;
            if (tile_i as i32 + 1) > height_res_fit {
                calculated_y = (tile_i as i32 / height_res_fit) * height_res;
            }
            let mut shared = false;
            for (p_i, p) in tile.pixels.iter().enumerate() {
                // println!("p_i: {}", p_i);
                if shared == false {
                    shared = true;
                    println!("loaded tile_i from second if: {}", tile_i);
                }
                if p_i as i32 <= (width_res - 1) {
                    calculated_x = p_i as i32 + ((tile_i as i32 % width_res_fit) * width_res);
                } else {
                    calculated_x = (p_i as i32 % width_res) + ((tile_i as i32 % width_res_fit) * width_res);
                    if (p_i as i32 % width_res) == 0 {
                        calculated_y += 1;
                    }
                }
                let pixel_colour = p.colour_rgba;
                screen[(calculated_y as usize * self.width + calculated_x as usize) * 4..(calculated_y as usize * self.width + calculated_x as usize) * 4 + 4].copy_from_slice(&pixel_colour);
            }
        }

        // Stop unnecessary re-renders
        self.needsReRender = false;
    }

    pub fn draw(&mut self, screen: &mut [u8]) {
        if self.needsReRender == false {
            return;
        }
        self.render_screen(screen);
    }

    pub fn update(&mut self) {
        self.tile_lifecycle();
    }

    pub fn randomise(&mut self) {
        println!("Randomising simulation grid...");
        // Clear Tiles
        self.tiles.clear();

        let mut rng = rand::thread_rng();

        // Generate tiles
        println!("Tile calculations:");
        let width_res_fit: i8 = (self.width / 32) as i8;
        let height_res_fit: i8 = (self.height / 32) as i8;
        println!("width_res_fit: {}", width_res_fit);
        println!("height_res_fit: {}", height_res_fit);
        let res_fit: i8 = width_res_fit * height_res_fit;
        println!("res_fit: {}", res_fit);
        for i in 0..res_fit {
            let random_terrain_type = match rng.gen_range(0..6) {
                0 => 'f',
                1 => 'w',
                2 => 'm',
                3 => 'h',
                4 => 'p',
                5 => 'w',
                _ => '.',
            };
            self.tiles.push(Tile::new(i as u8, i as u8, random_terrain_type));
        }
    }

    fn tile_lifecycle(&mut self) {
        for tile in self.tiles.iter_mut() {
            tile.update_tile();
        }
    }
}