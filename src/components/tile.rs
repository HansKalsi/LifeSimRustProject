use crate::components::pixel::Pixel;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Tile {
    pub x: u8,
    pub y: u8,
    pub terrain_type: char,
    pub pixels: Vec<Pixel>,
}

impl Tile {
    pub fn new(x: u8, y: u8, terrain_type: char) -> Self {
        // Setup pixels based on terrain type 
        let mut temp_pixels = vec![];
        // Create a 32x32 grid of pixels (resolution of a tile)
        for _ in 0..(32*32) {
            temp_pixels.push(Pixel::new(
                // Return a colour based on the terrain type
            match terrain_type {
                'f' => [255, 0, 0, 255], // Red
                'w' => [255, 125, 0, 255], // Orange
                'm' => [100, 100, 100, 255], // Gray
                'h' => [0, 255, 125, 255], // Teal
                'p' => [0, 255, 0, 255], // Green
                'w' => [0, 0, 255, 255], // Blue
                _ => [0, 0, 0, 255], // Black (default for empty tiles)
            }));
        }

        Self { x, y, terrain_type, pixels: temp_pixels }
    }
}