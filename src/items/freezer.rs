use crate::characters::ItemOf;
use crate::effects::freeze::FreezeEvent;
use crate::items::targeting::{TargetSelected, Targeting, TargetingSystems};
use crate::items::usable::UseEvent;
use bevy::prelude::*;

#[derive(Component)]
#[require(Targeting)]
pub struct Freezer;

pub fn freezer_used(
    trigger: Trigger<UseEvent>,
    q_freezer: Query<(Entity, &Targeting), (With<Freezer>, With<ItemOf>)>,
    mut commands: Commands,
    targeting_systems: Res<TargetingSystems>,
) {
    // The entity that triggered the event
    let freezer_entity = trigger.target();

    // Only continue if the item has the Freezer component
    let Ok((freezer_entity, targeting)) = q_freezer.get(freezer_entity) else {
        return;
    };

    commands
        .entity(freezer_entity)
        .observe(freezer_target_selected);

    let targeting_system = targeting_systems.system(targeting);
    commands.run_system_with(targeting_system, freezer_entity);
}

fn freezer_target_selected(
    trigger: Trigger<TargetSelected>,
    q_owner: Query<&ItemOf>,
    mut commands: Commands,
) {
    let owner = q_owner.get(trigger.source).unwrap().owner();
    commands.trigger(FreezeEvent::new(owner, trigger.target, trigger.source));
    commands.entity(trigger.observer()).despawn();
}
