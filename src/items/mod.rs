use crate::items::burner::burner_used;
use crate::items::healer::healer_used;
use crate::items::poisoner::poisoner_used;
use crate::items::shielder::shielder_used;
use crate::items::slower::slower_used;
use crate::items::targeting::targeting_startup;
use bevy::prelude::*;
use usable::tick_usable;
use weapons::weapon_used;
use crate::items::usable_with_targeted_effect::usable_with_targeted_effect_used;

pub mod armory;
mod burner;
mod freezer;
mod hastener;
mod healer;
mod poisoner;
mod shielder;
mod slower;
pub mod targeting;
pub mod usable;
mod usable_with_targeted_effect;
pub mod weapons;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(tick_usable)
            .add_observer(weapon_used)
            .add_observer(burner_used)
            .add_observer(poisoner_used)
            .add_observer(shielder_used)
            .add_observer(healer_used)
            .add_observer(slower_used)
            .add_observer(usable_with_targeted_effect_used)
            .add_systems(Startup, targeting_startup);
    }
}

#[derive(Component)]
#[require(Name)]
pub struct Item;
