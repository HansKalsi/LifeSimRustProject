#[derive(Clone, PartialEq, Debug, Default)]
pub struct Resource {
    pub resource_type: char,
    pub quantity: i16,
    pub colour_rgba: [u8; 4],
}

impl Resource {
    pub fn new(resource_type: char, colour_rgba: [u8; 4]) -> Self {
        Self { colour_rgba, quantity: 1, resource_type }
    }

    pub fn add_resource(&mut self, quantity_to_add: i16) {
        self.quantity += quantity_to_add;
    }

    pub fn remove_resource(&mut self, quantity_to_remove: i16) {
        self.quantity -= quantity_to_remove;
    }
}