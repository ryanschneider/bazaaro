use crate::effects::shield::{Shield, ShieldEvent};
use crate::items::usable::UseEvent;
use bevy::prelude::*;

pub fn shielder_used(
    trigger: Trigger<UseEvent>,
    query: Query<&ChildOf, With<Shield>>,
    mut commands: Commands,
) {
    let with = trigger.target();
    let Ok(child_of) = query.get(with) else {
        return;
    };

    let defender = child_of.parent();
    commands.trigger_targets(ShieldEvent::new(defender, with), defender);
}
