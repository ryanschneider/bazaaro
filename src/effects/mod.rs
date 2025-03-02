use crate::GameState;
use bevy::prelude::*;

pub mod attack;
pub mod burn;
pub mod freeze;
pub mod haste;
pub mod heal;
pub mod poison;
pub mod regen;
pub mod shield;
pub mod slow;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (burn::tick_burning, poison::tick_poisoned).run_if(in_state(GameState::Fight)),
        )
        .add_event::<poison::PoisonEvent>()
        .add_event::<heal::HealEvent>()
        .add_event::<slow::SlowEvent>()
        .add_event::<freeze::FreezeEvent>()
        .add_event::<haste::HasteEvent>()
        .add_observer(attack::on_attack)
        .add_observer(burn::on_burned)
        .add_observer(shield::on_shield)
        .add_observer(poison::on_poisoned)
        .add_observer(heal::on_heal)
        .add_observer(slow::on_slowed)
        .add_observer(freeze::on_frozen)
        .add_observer(haste::on_hastened);
    }
}
