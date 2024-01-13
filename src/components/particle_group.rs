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
}