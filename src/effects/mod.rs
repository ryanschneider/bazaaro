use crate::GameState;
use bevy::prelude::*;

pub mod attack;
pub mod burn;
pub mod heal;
pub mod poison;
pub mod regen;
pub mod shield;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (burn::tick_burning, poison::tick_poisoned,).run_if(in_state(GameState::Fight)),
        )
        .add_event::<poison::PoisonEvent>()
        .add_event::<heal::HealEvent>()
        .add_observer(attack::on_attack)
        .add_observer(burn::on_burned)
        .add_observer(shield::on_shield)
        .add_observer(poison::on_poisoned)
        .add_observer(heal::on_heal);
    }
}
