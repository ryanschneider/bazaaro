use crate::items::burner::burner_used;
use crate::items::shielder::shielder_used;
use bevy::prelude::*;
use usable::tick_usable;
use weapons::weapon_used;

pub mod armory;
mod burner;
mod shielder;
mod usable;
pub mod weapons;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(tick_usable)
            .add_observer(weapon_used)
            .add_observer(burner_used)
            .add_observer(shielder_used);
    }
}

#[derive(Component)]
pub struct Item;
