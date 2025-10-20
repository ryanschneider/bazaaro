use crate::characters::ItemOf;
use crate::effects::poison::{Poison, PoisonEvent};
use crate::fighting::Battle;
use crate::items::usable::UseEvent;
use bevy::prelude::*;

pub fn poisoner_used(
    trigger: On<UseEvent>,
    query: Query<&ItemOf, With<Poison>>,
    mut commands: Commands,
    battle: Res<Battle>,
) {
    let poisoned_with = trigger.event().entity;
    let Ok(item_of) = query.get(poisoned_with) else {
        return;
    };

    let attacker = item_of.owner();
    let defender = battle.opponent(attacker);
    commands
        .entity(defender)
        .trigger(|defender| PoisonEvent::new(attacker, defender, poisoned_with));
}
