#![deny(clippy::all)]
#![forbid(unsafe_code)]

use error_iter::ErrorIter as _;
use log::{error, log};
use pixels::{Error, Pixels, SurfaceTexture, wgpu::Color};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;
const PARTICLE_GROUPS_TO_GENERATE: usize = 4;

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * 3.0, HEIGHT as f64 * 3.0);
        WindowBuilder::new()
            .with_title("Hans' Life Simulator")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    // Customises the background colour
    // pixels.clear_color(Color::BLACK);

    let mut life = LifeGrid::new_random(WIDTH as usize, HEIGHT as usize, PARTICLE_GROUPS_TO_GENERATE as usize);
    let mut paused = false;

    event_loop.run(move |event, _, control_flow| {
        // The one and only event that winit_input_helper doesn't have for us...
        if let Event::RedrawRequested(_) = event {
            life.draw(pixels.frame_mut());
            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // For everything else, for let winit_input_helper collect events to build its state.
        // It returns `true` when it is time to update our game state and request a redraw.
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            if input.key_pressed(VirtualKeyCode::P) {
                paused = !paused;
            }
            if input.key_pressed_os(VirtualKeyCode::Space) {
                // Space is frame-step, so ensure we're paused
                paused = true;
            }
            if input.key_pressed(VirtualKeyCode::R) {
                // FIXME: randomise world state
                // life.randomize();
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            if !paused || input.key_pressed_os(VirtualKeyCode::Space) {
                life.update();
            }
            window.request_redraw();
        }
    });
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
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

#[derive(Clone, Debug, Default)]
struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    colour: Color,
}

impl Particle {
    fn new(x: f32, y: f32, vx: f32, vy: f32, colour: Color) -> Self {
        Self { x, y, vx, vy, colour }
    }
}

#[derive(Clone, Debug, Default)]
struct ParticleGroup {
    group: Vec<Particle>,
}

impl ParticleGroup {
    fn new(group: Vec<Particle>) -> Self {
        Self { group }
    }

    fn update_group(&mut self, modifed_group: Vec<Particle>) {
        self.group = modifed_group;
    }
}

#[derive(Clone, Debug, Default)]
struct Rule {
    particle_group_one: usize,
    particle_group_two: usize,
    g: f32,
}

impl Rule {
    fn new(particle_group_one: usize, particle_group_two: usize, g: f32) -> Self {
        Self { particle_group_one, particle_group_two, g }
    }
}

#[derive(Clone, Debug)]
struct LifeGrid {
    width: usize,
    height: usize,
    num_of_particle_groups: usize,
    particle_groups: Vec<ParticleGroup>,
    rules: Vec<Rule>,
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

    fn new_random(width: usize, height: usize, num_of_particle_groups: usize) -> Self {
        let mut result = Self::new_empty(width, height, num_of_particle_groups);
        result.generate_particles();
        result.randomise_rules();
        // result.randomize();
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
            // TODO: Allow number of particles generated per group to be set globally
            // Generate 100 particles
            for _ in 0..100 {
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

    // fn randomize(&mut self) {
    //     for _ in 0..3 {
    //         self.update();
    //     }
    // }

    fn trigger_rules(&mut self) {
        for r in self.rules.iter() {
            let mut modified_particles: Vec<Particle> = vec![];
            let pg1 = &self.particle_groups[r.particle_group_one].group;
            let pg2 = &self.particle_groups[r.particle_group_two].group;
            for (p1, p2) in pg1.iter().zip(pg2.iter()) {
                let mut fx: f32 = 0.0;
                let mut fy: f32 = 0.0;
                // particle two logic
                let dx = p1.x - p2.x;
                let dy = p1.y - p2.y;
                let d = (dx * dx + dy * dy).sqrt();
                if d > 0.0 && d < 80.0 {
                    let force = r.g * 1.0/d;
                    fx += force * dx;
                    fy += force * dy;
                }
                // after particle two logic
                let mut temp_p1 = p1.clone();
                temp_p1.vx = (temp_p1.vx + fx)*0.5;
                temp_p1.vy = (temp_p1.vy + fy)*0.5;
                temp_p1.x += temp_p1.vx;
                temp_p1.y += temp_p1.vy;
                if temp_p1.x < 0.0 || temp_p1.x > self.width as f32 {
                    temp_p1.vx *= -1.0;
                }
                if temp_p1.y < 0.0 || temp_p1.y > self.height as f32 {
                    temp_p1.vy *= -1.0;
                }
                modified_particles.push(Particle::new(temp_p1.x, temp_p1.y, temp_p1.vx, temp_p1.vy, temp_p1.colour));
            }
            self.particle_groups[r.particle_group_one].update_group(modified_particles);
        }
    }

    fn update(&mut self) {
        self.trigger_rules();
    }

    fn draw_particle(&self, particle: &Particle, screen: &mut [u8]) {
        let x = particle.x as usize;
        let y = particle.y as usize;
        let i = (y * self.width + x) * 4;
        screen[i] = particle.colour.r as u8;
        screen[i + 1] = particle.colour.g as u8;
        screen[i + 2] = particle.colour.b as u8;
        screen[i + 3] = particle.colour.a as u8;
    }

    fn draw(&self, screen: &mut [u8]) {
        println!("printing particle groups: {:?}", self.particle_groups.len());
        for p in self.particle_groups.iter() {
            for particle in p.group.iter() {
                self.draw_particle(particle, screen);
            }
        }
    }
}