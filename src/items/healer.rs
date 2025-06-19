use crate::effects::heal::{Heal, HealEvent};
use crate::items::usable::UseEvent;
use bevy::prelude::*;
use crate::characters::ItemOf;

pub fn healer_used(
    trigger: Trigger<UseEvent>,
    query: Query<&ItemOf, With<Heal>>,
    mut commands: Commands,
) {
    let heal_with = trigger.target();
    let Ok(item_of) = query.get(heal_with) else {
        return;
    };

    // Get the character using the heal item
    // (unlike other items, healer heals the user, not the opponent)
    let target = item_of.owner();

    // Trigger healing on the user themselves
    commands.trigger_targets(HealEvent::new(target, heal_with), target);
}
