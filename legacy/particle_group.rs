use crate::Particle;
use pixels::wgpu::Color;
use pixel_map::PixelMap;

#[derive(Clone, Debug, Default)]
pub struct ParticleGroup {
    pub group: Vec<Particle>,
}

impl ParticleGroup {
    pub fn new(group: Vec<Particle>) -> Self {
        Self { group }
    }

    pub fn lifecycle(&mut self) {
        let mut dead_particles: Vec<usize> = vec![];
        for (i, particle) in self.group.iter_mut().enumerate() {
            if !particle.lifecycle() {
                dead_particles.push(i);
            }
        }
        for i in dead_particles.iter().rev() {
            self.group.swap_remove(*i);
        }
    }

    //  ~58% weight | FIXME: most inefficent piece atm (need to revamp using the quadtree features)
    pub fn apply_rule(&mut self, rule_g: f32, other_group: Vec<Particle>, rule_effect: String, global_id_count: u32) {
        self.check_for_position_overlap(other_group.clone(), rule_effect, global_id_count);
        for particle in self.group.iter_mut() {
            let mut fx: f32 = 0.0;
            let mut fy: f32 = 0.0;
            for other_particle in other_group.iter() {
                let dx = particle.x - other_particle.x;
                let dy = particle.y - other_particle.y;
                let d = (dx * dx + dy * dy).sqrt();
                if d > 0.0 && d < 100.0 {
                    let force = rule_g * 1.0/d;
                    fx += force * dx;
                    fy += force * dy;
                }
            }
            particle.update_particle(fx, fy);
        }

        // for p in self.group.iter_mut() {
        //     // FIXME: need to rewrite code to use rect so I can use the quadtree features and check nearby rects for particles for efficiency
        //     let nearby_particles = pixel_map.visit_neighbor_pairs(&p.rect, );
        // }
    }

    // ~8% weight
    fn check_for_position_overlap(&mut self, other_group: Vec<Particle>, rule_effect: String, global_id_count: u32) {
        if rule_effect == "nothing" {
            return;
        }
        let mut spawned_children: Vec<Particle> = vec![];
        for particle in self.group.iter_mut() {
            for other_particle in other_group.iter() {
                let dx = (particle.x - other_particle.x).round();
                let dy = (particle.y - other_particle.y).round();
                if dx == 0.0 && dy == 0.0 {
                    if rule_effect == "spawn_children" {
                        spawned_children.append(&mut particle.spawn_children(global_id_count));
                    }
                }
            }
        }
        self.group.append(&mut spawned_children);
    }
}