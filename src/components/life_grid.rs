use crate::Particle;
use crate::ParticleGroup;
use crate::Rule;
use crate::MAX_PARTICLES_PER_GROUP;

use rand::Rng;
use pixels::wgpu::Color;

#[derive(Clone, Debug)]
pub struct LifeGrid {
    pub width: usize,
    pub height: usize,
    pub num_of_particle_groups: usize,
    pub particle_groups: Vec<ParticleGroup>,
    pub rules: Vec<Rule>,
}

impl LifeGrid {
    fn new_empty(width: usize, height: usize, num_of_particle_groups: usize) -> Self {
        assert!(width != 0 && height != 0);
        Self {
            width,
            height,
            num_of_particle_groups,
            particle_groups: vec![],
            rules: vec![],
        }
    }

    pub fn new_random(width: usize, height: usize, num_of_particle_groups: usize) -> Self {
        let mut result = Self::new_empty(width, height, num_of_particle_groups);
        result.generate_particles();
        result.randomise_rules();
        result
    }

    fn randomise_rgb_colours(&mut self) -> Vec<Color> {
        let mut rng: randomize::PCG32 = generate_seed().into();
        let mut colours: Vec<Color> = vec![];

        for _ in 0..self.num_of_particle_groups {
            let mut colour: Color = Color::default();
            colour.r = (rng.next_u32() % 256 as u32) as f64;
            colour.g = (rng.next_u32() % 256 as u32) as f64;
            colour.b = (rng.next_u32() % 256 as u32) as f64;
            colour.a = 0xff as f64;
            colours.push(colour);
        }
        colours
    }

    fn generate_particles(&mut self) {
        let mut rng: randomize::PCG32 = generate_seed().into();
        let mut particle_groups: Vec<ParticleGroup> = vec![];
        let colours: Vec<Color> = self.randomise_rgb_colours();

        for c in colours.iter() {
            let mut particles: Vec<Particle> = vec![];
            let particles_to_generate = rng.next_u32() % MAX_PARTICLES_PER_GROUP as u32;
            for _ in 0..particles_to_generate {
                let x = randomize::f32_half_open_right(rng.next_u32()) * self.width as f32;
                let y = randomize::f32_half_open_right(rng.next_u32()) * self.height as f32;
                let vx = 0.0;
                let vy = 0.0;
                particles.push(Particle::new(x, y, vx, vy, *c));
            }
            particle_groups.push(ParticleGroup::new(particles));
        }

        self.particle_groups = particle_groups;
    }

    fn randomise_rules(&mut self) {
        let mut rng: randomize::PCG32 = generate_seed().into();
        let mut rules: Vec<Rule> = vec![];

        for particle_group_one in 0..self.num_of_particle_groups {
            for particle_group_two in 0..self.num_of_particle_groups {
                let g = randomize::f32_half_open_right(rng.next_u32());
                rules.push(Rule::new(particle_group_one, particle_group_two, g));
            }
        }

        self.rules = rules;
    }

    pub fn randomize(&mut self) {
        self.generate_particles();
        self.randomise_rules();
        for _ in 0..3 {
            self.update();
        }
    }

    fn trigger_rules(&mut self) {
        for r in self.rules.iter() {
            let mut rng = rand::thread_rng();
            let mut modified_particles: Vec<Particle> = vec![];
            let pg1 = &self.particle_groups[r.particle_group_one].group;
            let pg2 = &self.particle_groups[r.particle_group_two].group;
            for p1 in pg1.iter() {
                let mut fx: f32 = 0.0;
                let mut fy: f32 = 0.0;
                // particle two logic
                for p2 in pg2.iter() {
                    let dx = p1.x - p2.x;
                    let dy = p1.y - p2.y;
                    let d = (dx * dx + dy * dy).sqrt();
                    if d > 0.0 && d < 100.0 {
                        let force = r.g * 1.0/d;
                        fx += force * dx;
                        fy += force * dy;
                    }
                }
                // after particle two logic
                let mut temp_p1 = p1.clone();
                temp_p1.vx = (temp_p1.vx + fx)*0.5;
                temp_p1.vy = (temp_p1.vy + fy)*0.5;
                temp_p1.x += temp_p1.vx;
                temp_p1.y += temp_p1.vy;
                // the rng.gen_range lines appear to cause something akin to mutation and result in constant complexity
                if temp_p1.x < 0.0 || temp_p1.x > self.width as f32 {
                    temp_p1.x = rng.gen_range(0.0..self.width as f32);
                    temp_p1.vx *= -1.0;
                }
                if temp_p1.y < 0.0 || temp_p1.y > self.height as f32 {
                    temp_p1.y = rng.gen_range(0.0..self.height as f32);
                    temp_p1.vy *= -1.0;
                }

                
                modified_particles.push(Particle::new(temp_p1.x, temp_p1.y, temp_p1.vx, temp_p1.vy, temp_p1.colour));
            }
            self.particle_groups[r.particle_group_one].update_group(modified_particles);
        }
    }

    pub fn update(&mut self) {
        self.trigger_rules();
    }

    fn draw_particle(&self, particle: &Particle, screen: &mut [u8]) {
        let x = particle.x as usize;
        let y = particle.y as usize;
        let screen_size = screen.len() - 4;
        let i = ((y * self.height + x) * 4).clamp(0, screen_size);
        println!("i: {}", i);
        screen[i] = particle.colour.r as u8;
        screen[i + 1] = particle.colour.g as u8;
        screen[i + 2] = particle.colour.b as u8;
        screen[i + 3] = particle.colour.a as u8;
    }

    pub fn draw(&self, screen: &mut [u8]) {
        for pixel in screen.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[0, 0, 0, 0]);
        }

        for p in self.particle_groups.iter() {
            for particle in p.group.iter() {
                self.draw_particle(particle, screen);
            }
        }
    }
}

/// Generate a pseudorandom seed for the game's PRNG.
fn generate_seed() -> (u64, u64) {
    use byteorder::{ByteOrder, NativeEndian};
    use getrandom::getrandom;

    let mut seed = [0_u8; 16];

    getrandom(&mut seed).expect("failed to getrandom");

    (
        NativeEndian::read_u64(&seed[0..8]),
        NativeEndian::read_u64(&seed[8..16]),
    )
}