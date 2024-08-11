use super::particle::Particle;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Organism {
    pub x: u8,
    pub y: u8,
    pub pixels: Vec<Particle>,
}

impl Organism {
    pub fn new(x: u8, y: u8, pixels: Vec<Particle>) -> Self {
        Self { x, y, pixels }
    }

    pub fn randomise_pixel_colours(&mut self) {
        for pixel in self.pixels.iter_mut() {
            pixel.randomise_pixel_colour();
        }
    }
}