use crate::effects::poison::{Poison, PoisonEvent};
use crate::fighting::Battle;
use crate::items::usable::UseEvent;
use bevy::prelude::*;

pub fn poisoner_used(
    trigger: Trigger<UseEvent>,
    query: Query<&ChildOf, With<Poison>>,
    mut commands: Commands,
    battle: Res<Battle>,
) {
    let poisoned_with = trigger.target();
    let Ok(child_of) = query.get(poisoned_with) else {
        return;
    };

    let attacker = child_of.parent();
    let defender = battle.opponent(attacker);
    commands.trigger_targets(
        PoisonEvent::new(attacker, defender, poisoned_with),
        defender,
    );
}
