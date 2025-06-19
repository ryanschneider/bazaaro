use crate::effects::burn::{Burn, BurnEvent};
use crate::fighting::Battle;
use crate::items::usable::UseEvent;
use bevy::prelude::*;

pub fn burner_used(
    trigger: Trigger<UseEvent>,
    query: Query<&ChildOf, With<Burn>>,
    mut commands: Commands,
    battle: Res<Battle>,
) {
    let burned_with = trigger.target();
    let Ok(child_of) = query.get(burned_with) else {
        return;
    };

    let attacker = child_of.parent();
    let defender = battle.opponent(attacker);
    commands.trigger_targets(BurnEvent::new(attacker, defender, burned_with), defender);
}
