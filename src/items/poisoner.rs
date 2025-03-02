use crate::effects::poison::{Poison, PoisonEvent};
use crate::fighting::Battle;
use crate::items::usable::UseEvent;
use bevy::prelude::*;

pub fn poisoner_used(
    trigger: Trigger<UseEvent>,
    query: Query<&Parent, With<Poison>>,
    mut commands: Commands,
    battle: Res<Battle>,
) {
    let poisoned_with = trigger.entity();
    let Ok(parent) = query.get(poisoned_with) else {
        return;
    };

    let attacker = parent.get();
    let defender = battle.opponent(attacker);
    commands.trigger_targets(
        PoisonEvent::new(attacker, defender, poisoned_with),
        defender,
    );
}
