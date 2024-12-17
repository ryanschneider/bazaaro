use bevy::prelude::*;

#[derive(Component)]
pub struct Burn {
    pub amount: u32,
}

impl Burn {
    pub fn new(amount: u32) -> Self {
        Burn { amount }
    }
}
