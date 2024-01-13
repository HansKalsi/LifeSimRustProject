use pixels::wgpu::Color;

#[derive(Clone, Debug, Default)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub colour: Color,
}

impl Particle {
    pub fn new(x: f32, y: f32, vx: f32, vy: f32, colour: Color) -> Self {
        Self { x, y, vx, vy, colour }
    }
}