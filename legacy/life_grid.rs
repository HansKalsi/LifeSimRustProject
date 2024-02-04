use crate::Particle;
use crate::ParticleGroup;
use crate::Rule;
use crate::MAX_PARTICLES_PER_GROUP;
use crate::generate_seed;

use pixels::wgpu::Color;
use pixel_map::PixelMap;
use bevy_math::UVec2;
use pixel_map::PNode;

#[derive(Clone, Debug)]
pub struct LifeGrid {
    pub width: usize,
    pub height: usize,
    pub global_id_count: u32,
    pub num_of_particle_groups: usize,
    pub colours: Vec<Color>,
    pub rules: Vec<Rule>,
    pub pixel_map: PixelMap<Particle>,
    pub live_particle_count: i32,
    pub runs_with_life: i32,
}

impl LifeGrid {
    fn new_empty(width: usize, height: usize, num_of_particle_groups: usize, pixel_map: PixelMap<Particle>) -> Self {
        assert!(width != 0 && height != 0);
        Self {
            width,
            height,
            num_of_particle_groups,
            colours: vec![],
            rules: vec![],
            pixel_map,
            global_id_count: 1,
            live_particle_count: 0,
            runs_with_life: 0,
        }
    }

    pub fn new_random(width: usize, height: usize, num_of_particle_groups: usize) -> Self {
        let pixel_map = PixelMap::<Particle>::new(
            &UVec2{x: width as u32, y: height as u32}, // size of the pixel map
            Particle::default(), // initial value of each pixel
            1, // pixel size
        );
        let mut result = Self::new_empty(width, height, num_of_particle_groups, pixel_map);
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
        self.colours = self.randomise_rgb_colours();

        for c in self.colours.iter() {
            let particles_to_generate = rng.next_u32() % MAX_PARTICLES_PER_GROUP as u32;
            for id in 1..particles_to_generate {
                self.global_id_count += 1;
                let x = randomize::f32_half_open_right(rng.next_u32()) * self.width as f32;
                let y = randomize::f32_half_open_right(rng.next_u32()) * self.height as f32;
                let initial_cords = UVec2{ x: x as u32, y: y as u32 };
                self.pixel_map.set_pixel(initial_cords, Particle::new(id, x, y, 0.0, 0.0, *c, 1));
            }
        }
    }

    fn randomise_rules(&mut self) {
        self.rules = vec![];
        for particle_group_one in 0..self.num_of_particle_groups {
            for particle_group_two in 0..self.num_of_particle_groups {
                if particle_group_one == particle_group_two {
                    self.rules.push(Rule::new(self.colours[particle_group_one], self.colours[particle_group_two], false));
                    continue;
                }
                self.rules.push(Rule::new(self.colours[particle_group_one], self.colours[particle_group_two], true));
            }
        }
    }

    pub fn randomize(&mut self) {
        self.runs_with_life = 0;
        self.generate_particles();
        self.randomise_rules();
        for _ in 0..3 {
            self.update();
        }
    }

    fn trigger_rules(&mut self) {
        let mut temp_particle_groups_by_colour: Vec<ParticleGroup> = vec![];
        let mut temp_colour_tracker: Vec<Color> = vec![];

        // Visit all nodes (particles)
        self.pixel_map.visit(|p, _rect| {
            if p.value().id != 0 {
                for c in self.colours.iter() {
                    if p.value().colour == *c {
                        // Check temp_colour_tracker for index if it exists
                        if let Some(index) = temp_colour_tracker.iter().position(|&color| color == *c) {
                            temp_particle_groups_by_colour[index].group.push(p.value().clone());
                        } else {
                            // If it doesn't exist, create a new ParticleGroup and push it to particle_groups_by_colour
                            let mut new_particle_group = ParticleGroup::default();
                            new_particle_group.group.push(p.value().clone());
                            temp_particle_groups_by_colour.push(new_particle_group);
                            temp_colour_tracker.push(*c);
                        }
                        break;
                    }
                }
            }
        });

        // if self.pixel_map.dirty() {
        //     println!("QUADTREE HAS CHANGES");
        // }
        // self.pixel_map.visit_dirty(visitor);
        // self.pixel_map.drain_dirty(|p| {
        //     if p.value().id != 0 {
        //         println!("Particle {} has changed", p.value().id)
        //     }
        // });
        
        // Remove rules that don't have any particles left
        let mut live_rules: Vec<Rule> = vec![];
        for r in self.rules.iter() {
            let particle_group_one_colour = r.particle_group_one_colour;
            let particle_group_two_colour = r.particle_group_two_colour;

            if temp_colour_tracker.contains(&particle_group_one_colour) && temp_colour_tracker.contains(&particle_group_two_colour) {
                live_rules.push(r.clone());
            }
        }
        self.rules = live_rules;

        // Apply rules
        for r in self.rules.iter() {
            let particle_group_one_colour = r.particle_group_one_colour;
            let particle_group_two_colour = r.particle_group_two_colour;

            let particle_group_one_index = temp_colour_tracker.iter().position(|&color| color == particle_group_one_colour);
            let particle_group_two_index = temp_colour_tracker.iter().position(|&color| color == particle_group_two_colour);

            let temp_group_two_clone = temp_particle_groups_by_colour[particle_group_two_index.unwrap()].group.clone();
            temp_particle_groups_by_colour[particle_group_one_index.unwrap()].apply_rule(r.g, temp_group_two_clone, r.effect.clone(), self.global_id_count);
        }

        // Trigger lifecycle events
        let temp_final_particle_groups = self.lifecycle_events(temp_particle_groups_by_colour);

        // Identify quadtree nodes to update
        let mut temp_update_pixel_map: Vec<Particle> = vec![];
        self.pixel_map.visit(|p_node, _rect| {
            if p_node.value().id != 0 {
                let mut found_node = false;
                for pg in temp_final_particle_groups.iter() {
                    for p in pg.group.iter() {
                        if p_node.value().id == p.id {
                            temp_update_pixel_map.push(*p);
                            found_node = true;
                            break;
                        }
                    }
                    if found_node {
                        break;
                    }
                }
            }
        });

        self.pixel_map.clear(Particle::empty());

        // Update quadtree
        for p in temp_update_pixel_map.iter() {
            // Update node particle
            let current_cords = UVec2{ x: p.x as u32, y: p.y as u32 };
            self.pixel_map.set_pixel(current_cords, *p);
        }
        self.live_particle_count = temp_update_pixel_map.len() as i32;
        self.runs_with_life += 1;
    }

    fn lifecycle_events(&mut self, mut temp_particle_groups: Vec<ParticleGroup>) -> Vec<ParticleGroup> {
        for pg in temp_particle_groups.iter_mut() {
            pg.lifecycle();
        }
        temp_particle_groups
    }

    pub fn update(&mut self) {
        self.trigger_rules();
    }

    fn draw_particle(&self, particle: &PNode<Particle>, screen: &mut [u8]) {
        let x = particle.value().x as usize;
        let y = particle.value().y as usize;
        let screen_size = screen.len() - 4;
        let i = ((y * self.height + x) * 4).clamp(0, screen_size);
        screen[i] = particle.value().colour.r as u8;
        screen[i + 1] = particle.value().colour.g as u8;
        screen[i + 2] = particle.value().colour.b as u8;
        screen[i + 3] = particle.value().colour.a as u8;
    }

    // Need rect, node, screen to be passed when draw_particle
    // ~16% weight | FIXME: second most inefficent piece atm
    pub fn draw(&mut self, screen: &mut [u8]) {
        // Clear the canvas
        for pixel in screen.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[0, 0, 0, 0]);
        }

        // Print particle amount to console
        if self.live_particle_count != 0 {
            println!("amount of particles: {}", self.live_particle_count);
            println!("runs with life: {}", self.runs_with_life);
        }

        // Visit all leaf nodes
        self.pixel_map.visit(|node, _rect| {
            if node.value().id != 0 {
                self.draw_particle(node, screen);
            }
        });
    }
}
