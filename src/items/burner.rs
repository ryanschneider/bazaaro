use crate::characters::ItemOf;
use crate::effects::burn::{Burn, BurnEvent};
use crate::fighting::Battle;
use crate::items::usable::UseEvent;
use bevy::prelude::*;

pub fn burner_used(
    trigger: Trigger<UseEvent>,
    query: Query<&ItemOf, With<Burn>>,
    mut commands: Commands,
    battle: Res<Battle>,
) {
    let burned_with = trigger.target();
    let Ok(item_of) = query.get(burned_with) else {
        return;
    };

    let attacker = item_of.owner();
    let defender = battle.opponent(attacker);
    commands.trigger_targets(BurnEvent::new(attacker, defender, burned_with), defender);
}
