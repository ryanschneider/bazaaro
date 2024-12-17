use bevy::prelude::Component;

#[derive(Default, Component)]
pub struct Shielded(u32);

impl Shielded {
    pub fn absorb(&mut self, dmg: u32) -> u32 {
        if dmg > self.0 {
            let remaining = dmg - self.0;
            self.0 = 0;
            remaining
        } else {
            0
        }
    }
}
