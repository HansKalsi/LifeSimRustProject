use crate::Particle;

#[derive(Clone, Debug, Default)]
pub struct ParticleGroup {
    pub group: Vec<Particle>,
}

impl ParticleGroup {
    pub fn new(group: Vec<Particle>) -> Self {
        Self { group }
    }

    pub fn update_group(&mut self, modifed_group: Vec<Particle>) {
        self.group = modifed_group;
    }

    pub fn apply_rule(&mut self, rule_g: f32, other_group: Vec<Particle>) {
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
    }
}