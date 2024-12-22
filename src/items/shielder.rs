use crate::effects::shield::{Shield, ShieldEvent};
use crate::items::usable::UseEvent;
use bevy::prelude::*;

pub fn shielder_used(
    trigger: Trigger<UseEvent>,
    query: Query<&Parent, With<Shield>>,
    mut commands: Commands,
) {
    let with = trigger.entity();
    let Ok(parent) = query.get(with) else {
        return;
    };

    let defender = parent.get();
    commands.trigger_targets(ShieldEvent::new(defender, with), defender);
}
