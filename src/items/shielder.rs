use crate::characters::ItemOf;
use crate::effects::shield::{Shield, ShieldEvent};
use crate::items::usable::UseEvent;
use bevy::prelude::*;

pub fn shielder_used(
    trigger: On<UseEvent>,
    query: Query<&ItemOf, With<Shield>>,
    mut commands: Commands,
) {
    let with = trigger.event().entity;
    let Ok(item_of) = query.get(with) else {
        return;
    };

    let defender = item_of.owner();
    commands.entity(defender).trigger(|defender| ShieldEvent::new(defender, with));
}
