use pixels::wgpu::Color;
use rand::Rng;
use pixel_map::PNode;
use bevy_math::URect;

use crate::{WIDTH, HEIGHT};

#[derive(Clone, Debug, Default)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub colour: Color, // TODO: fade alpha to 0 as life force decreases
    // TODO: Add lifecycle logic for birth/survival/death of particles
    /// represents how many children this particle will spawn
    pub birth_rate: i8,
    // pub birth_cooldown: i8 // TODO: add cooldown to prevent spawning too many children in a short period of time
    pub life_force: i8,
    // FIXME: Need to figure out how to add a node to the particle so it can be used for the quadtree (OR DO DIFFERENTLY (LIKELY USING VISIT IN SOME WAY))
    // pub node: PNode,
}

impl Particle {
    pub fn new(x: f32, y: f32, vx: f32, vy: f32, colour: Color, birth_rate: i8) -> Self {
        let mut rng = rand::thread_rng();
        Self { x, y, vx, vy, colour, birth_rate, life_force: rng.gen_range(50.0..100.0) as i8 }
    }

    pub fn update_particle(&mut self, fx: f32, fy: f32) {
        let mut rng = rand::thread_rng();
        self.vx = (self.vx + fx)*0.5;
        self.vy = (self.vy + fy)*0.5;
        self.x += self.vx;
        self.y += self.vy;
        // the rng.gen_range lines appear to cause something akin to mutation and result in constant complexity
        if self.x < 0.0 || self.x > WIDTH as f32 {
            self.x = rng.gen_range(0.0..WIDTH as f32);
            self.vx *= -1.0;
        }
        if self.y < 0.0 || self.y > HEIGHT as f32 {
            self.y = rng.gen_range(0.0..HEIGHT as f32);
            self.vy *= -1.0;
        }
    }

    pub fn lifecycle(&mut self) -> bool {
        if self.is_alive() {
            self.reduce_life_force(1);
            return true; // is alive (was active)
        } else {
            return false; // is dead (was inactive)
        }
    }

    fn is_alive(&mut self) -> bool {
        if self.life_force <= 0 {
            return false;
        }
        return true;
    }

    // TODO: add rule effect for gaining life force (eg consuming life force of another particle)
    pub fn add_life_force(&mut self, life_force_to_add: i8) {
        self.life_force += life_force_to_add;
    }

    pub fn reduce_life_force(&mut self, life_force_to_reduce: i8) {
        self.life_force -= life_force_to_reduce;
    }

    pub fn spawn_children(&mut self) -> Vec<Particle> {
        let mut rng = rand::thread_rng();
        let mut children: Vec<Particle> = vec![];
        // TODO: make offsets a property of the particle (to allow for random variation) 
        let birth_offset_x: f32 = rng.gen_range(-100.0..100.0 as f32);
        let birth_offset_y: f32 = rng.gen_range(-100.0..100.0 as f32);
        for _ in 0..self.birth_rate {
            children.push(Particle::new(self.x + birth_offset_x, self.y + birth_offset_y, 0.0, 0.0, self.colour, self.birth_rate));
        }
        // self.reduce_life_force(20); // cost of energy to spawn children
        children // return vector so it can be added to the parents particle group
    }
}