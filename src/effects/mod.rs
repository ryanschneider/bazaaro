use bevy::prelude::*;

pub mod attack;
pub mod burn;
pub mod poison;
pub mod regen;
mod shield;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(burn::tick_burning);
        app.add_observer(attack::on_attack);
        app.add_observer(burn::on_burned);
    }
}

#[derive(Default, Bundle)]
pub struct DefaultEffects {
    shielded: shield::Shielded,
    regeneration: regen::Regeneration,
    burning: burn::Burning,
    poisoned: poison::Poisoned,
}
