use crate::effects::burn::Burn;
use crate::fighting::{Battle, BurnEvent};
use crate::items::usable::UseEvent;
use bevy::prelude::*;

pub fn burner_used(
    trigger: Trigger<UseEvent>,
    query: Query<&Parent, With<Burn>>,
    mut commands: Commands,
    battle: Res<Battle>,
) {
    let burned_with = trigger.entity();
    let Ok(parent) = query.get(burned_with) else {
        return;
    };

    let attacker = parent.get();
    let defender = battle.opponent(attacker);
    commands.trigger_targets(BurnEvent::new(attacker, defender, burned_with), defender);
}
