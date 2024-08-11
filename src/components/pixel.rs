#[derive(Clone, PartialEq, Debug, Default)]
pub struct Pixel {
    pub colour_rgba: [u8; 4],
}

impl Pixel {
    pub fn new(colour_rgba: [u8; 4]) -> Self {
        Self { colour_rgba }
    }
}