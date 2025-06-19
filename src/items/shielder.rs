use crate::effects::shield::{Shield, ShieldEvent};
use crate::items::usable::UseEvent;
use bevy::prelude::*;
use crate::characters::ItemOf;

pub fn shielder_used(
    trigger: Trigger<UseEvent>,
    query: Query<&ItemOf, With<Shield>>,
    mut commands: Commands,
) {
    let with = trigger.target();
    let Ok(item_of) = query.get(with) else {
        return;
    };

    let defender = item_of.owner();
    commands.trigger_targets(ShieldEvent::new(defender, with), defender);
}
