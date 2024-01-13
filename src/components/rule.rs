#[derive(Clone, Debug, Default)]
pub struct Rule {
    pub particle_group_one: usize,
    pub particle_group_two: usize,
    pub g: f32,
}

impl Rule {
    pub fn new(particle_group_one: usize, particle_group_two: usize, g: f32) -> Self {
        Self { particle_group_one, particle_group_two, g }
    }
}