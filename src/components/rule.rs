use crate::generate_seed;
use rand::Rng;

#[derive(Clone, Debug, Default)]
pub struct Rule {
    pub particle_group_one: usize,
    pub particle_group_two: usize,
    pub g: f32,
    // TODO: Add further logic to rules to allow for interaction/effects between particle groups
    pub effect: String,
}

impl Rule {
    pub fn new(particle_group_one: usize, particle_group_two: usize, effect_allowed: bool) -> Self {
        let mut rng: randomize::PCG32 = generate_seed().into();
        Self { particle_group_one, particle_group_two, g: randomize::f32_half_open_right(rng.next_u32()), effect: Self::assign_random_effect(effect_allowed) }
    }

    fn assign_random_effect(effect_allowed: bool) -> String {
        if !effect_allowed {
            return String::from("nothing");
        }
        let mut rng = rand::thread_rng();
        let random_number: f32 = rng.gen_range(0.0..1.0);
        if random_number < 0.5 {
            return String::from("spawn_children")
        } else {
            return String::from("nothing")
        }
    }
}
