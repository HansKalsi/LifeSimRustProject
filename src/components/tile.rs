use rand::Rng;
use crate::components::pixel::Pixel;
use crate::components::resource::Resource;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Tile {
    pub x: u8,
    pub y: u8,
    pub terrain_type: char,
    pub pixels: Vec<Pixel>,
    pub resources: Vec<Resource>,
    pub generates_resource: char,
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
        // Possibly generate a resource dependant on terrain type and random chance
        let mut rng = rand::thread_rng();
        let random_number: u8 = rng.gen_range(0..100);
        let mut generates_resource = 'n';
        match terrain_type {
            'f' => {
                // Generate food
                generates_resource = 'f';
            },
            'w' => {
                // Generate wood
                generates_resource = 'w';
            },
            'm' => {
                // Generate iron, stone or nothing
                if random_number < 50 {
                    generates_resource = 'i';
                } else if random_number < 75 {
                    generates_resource = 's';
                }
            },
            'h' => {
                // Generate stone or nothing
                if random_number < 50 {
                    generates_resource = 's';
                }
            },
            'p' => {
                // Generate nothing
                if random_number < 33 {
                    generates_resource = 'h';
                }
            },
            'w' => {
                // Generate a water
                if random_number < 33 {
                    generates_resource = 'p';
                }
            },
            _ => {
                // Do nothing
            }
        }

        Self { x, y, terrain_type, pixels: temp_pixels, resources: vec![], generates_resource }
    }

    pub fn update_tile(&mut self) {
        self.generate_resource();
        println!("Tile Resources: {:?}", self.resources);
    }

    fn generate_resource(&mut self) {
        // Generate a resource on the tile
        for r in self.resources.iter_mut() {
            if r.resource_type == self.generates_resource {
                r.add_resource(1);
                return;
            }
        };
        match self.generates_resource {
            'f' => {
                // Generate food
                self.resources.push(Resource::new('f', [0, 0, 0, 255]));
            },
            'w' => {
                // Generate wood
                self.resources.push(Resource::new('w', [0, 0, 0, 255]));
            },
            'i' => {
                // Generate iron
                self.resources.push(Resource::new('i', [0, 0, 0, 255]));
            },
            's' => {
                // Generate stone
                self.resources.push(Resource::new('s', [0, 0, 0, 255]));
            },
            'h' => {
                // Generate horse
                self.resources.push(Resource::new('h', [0, 0, 0, 255]));
            },
            'p' => {
                // Generate fish
                self.resources.push(Resource::new('p', [0, 0, 0, 255]));
            },
            _ => {
                // Do nothing
            }
        }
    }
}