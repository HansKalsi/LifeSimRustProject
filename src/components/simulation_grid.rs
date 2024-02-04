use bevy_math::UVec2;
use pixels::wgpu::Color;
use pixel_map::PixelMap;
use crate::generate_seed;
// use pixel_map::PNode;

use crate::components::particle::Particle;

pub struct SimGrid {
    pub width: usize,
    pub height: usize,
    pub pixel_size: usize,
    // pub pixel_map: PixelMap<Particle>,
    pub x: u8,
    pub y: u8,
    pub pixels: Vec<Particle>,
    pub timespace: Vec<(usize, usize)>,
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
            // pixel_map: PixelMap::<Particle>::new(
            //     &UVec2{x: width as u32, y: height as u32}, // size of the pixel map
            //     Particle::default(), // initial value of each pixel
            //     1, // pixel size
            // ),
            pixels: vec![Particle::default(); width * height],
            timespace: vec![],
        }
    }

    pub fn generate_timespace(&mut self, screen: &mut [u8]) {
        println!("timespace: {}", self.timespace.len());
        println!("width: {}", self.width);
        println!("height: {}", self.height);
        println!("width * height: {}", self.width * self.height);
        println!("x: {}", self.x);
        println!("y: {}", self.y);

        for y in 0..self.height {
            for x in 0..self.width {
                // let pixel = self.pixel_map.get_pixel(UVec2{x: x as u32, y: y as u32});
                // let pixel_colour = pixel.unwrap().pixel_colour_rgba;
                let pixel_colour = [255, 255, 255, 255];
                let pixel_colour = [pixel_colour[0], pixel_colour[1], pixel_colour[2], pixel_colour[3]];
                screen[(y * self.width + x) * 4..(y * self.width + x) * 4 + 4].copy_from_slice(&pixel_colour);
            }
        }
    }

    // Need rect, node, screen to be passed when draw_particle
    // ~16% weight | FIXME: second most inefficent piece atm
    pub fn draw(&mut self, screen: &mut [u8]) {
        // Clear the canvas
        let mut coutner = 0;
        // Array of [u8; 4] equal to the amount of chunks in screen
        if self.timespace.len() == 0 {
            self.generate_timespace(screen);
            println!("screen: {}", screen.chunks_exact_mut(4).len());
        }

        // for (pixel, cell) in screen.chunks_exact_mut(4).zip(self.timespace.iter()) {
        //     coutner+=1;
        //     pixel.copy_from_slice(cell.pixel_colour_rgba.as_ref());
        // }
        // println!("counter: {}", coutner);
        // println!("chunks: {}", coutner*4);

        // // Print particle amount to console
        // if self.live_particle_count != 0 {
        //     println!("amount of particles: {}", self.live_particle_count);
        //     println!("runs with life: {}", self.runs_with_life);
        // }

        // // Visit all leaf nodes
        // self.pixel_map.visit(|node, _rect| {
        //     if node.value().id != 0 {
        //         self.draw_particle(node, screen);
        //     }
        // });
    }

    pub fn update(&mut self) {    
        // each colour in rgba represent a different rule, this will allow nearby particles to interact and allow for local effects
        // let mut tracker = 0;
        // let temp_timespace_copy = self.timespace.clone(); 
        // for p in temp_timespace_copy.iter() {
        //     tracker+=1;
        //     if self.timespace.len() == tracker {
        //         break;
        //     }
        //     let right_neighbour = self.timespace.get(tracker);
        //     // Compare rgb values and drag each other toward an average for now
        //     let r_avg = (p.pixel_colour_rgba[0] + right_neighbour.unwrap().pixel_colour_rgba[0])/2;
        //     let g_avg = (p.pixel_colour_rgba[1] + right_neighbour.unwrap().pixel_colour_rgba[1])/2;
        //     let b_avg = (p.pixel_colour_rgba[2] + right_neighbour.unwrap().pixel_colour_rgba[2])/2;

        //     self.timespace[tracker - 1].pixel_colour_rgba[0] = r_avg;
        //     self.timespace[tracker - 1].pixel_colour_rgba[1] = g_avg;
        //     self.timespace[tracker - 1].pixel_colour_rgba[2] = b_avg;


        //     // if left neighbour is not null
        //     if tracker == 1 {
        //         println!("tracker: {}", tracker);
        //         continue;
        //     }
        //     let left_neighbour = self.timespace.get(tracker - 2);
        //     // Compare rgb values and drag each other toward an average for now
        //     let r_avg = (p.pixel_colour_rgba[0] + left_neighbour.unwrap().pixel_colour_rgba[0])/2;
        //     let g_avg = (p.pixel_colour_rgba[1] + left_neighbour.unwrap().pixel_colour_rgba[1])/2;
        //     let b_avg = (p.pixel_colour_rgba[2] + left_neighbour.unwrap().pixel_colour_rgba[2])/2;

        //     self.timespace[tracker - 1].pixel_colour_rgba[0] = r_avg;
        //     self.timespace[tracker - 1].pixel_colour_rgba[1] = g_avg;
        //     self.timespace[tracker - 1].pixel_colour_rgba[2] = b_avg;
        // }
    }

    pub fn randomise(&mut self) {
        // for p in self.timespace.iter_mut() {
        //     p.randomise_pixel_colour();
        // }
        // self.pixel_map.visit(|node, _rect| {
        //     if node.value().id != 0 {
        //         node.value().x += 1.0;
        //         node.value().randomise_pixel_colour();
        //     }
        // });
    }
}