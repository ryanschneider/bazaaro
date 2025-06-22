use crate::characters::ItemOf;
use crate::items::targeting::{TargetSelected, Targeting, TargetingSystems};
use crate::items::usable::UseEvent;
use bevy::prelude::{Commands, Component, Entity, Query, Res, Trigger, With};

#[derive(Debug, Clone, Copy)]
pub struct EffectContext {
    pub owner: Entity,   // Who's using the item
    pub target: Entity,  // What they're targeting
    pub source: Entity,  // The item itself
}

// Instead of a generic component, use a marker component with a boxed closure
#[derive(Component)]
#[require(Targeting)]
pub struct UsableWithTargetedEffect {
    pub effect_fn: fn(EffectContext, &mut Commands),
}

impl UsableWithTargetedEffect {
    pub fn new(effect_fn: fn(EffectContext, &mut Commands)) -> Self
    {
        Self {
            effect_fn,
        }
    }
}

pub fn usable_with_targeted_effect_used(
    trigger: Trigger<UseEvent>,
    q_item: Query<(Entity, &Targeting, &UsableWithTargetedEffect), With<ItemOf>>,
    mut commands: Commands,
    targeting_systems: Res<TargetingSystems>,
) {
    // The entity that triggered the event
    let item_entity = trigger.target();

    // Only continue if the item has the UsableWithTargetedEffect component
    let Ok((item_entity, targeting, _effect)) = q_item.get(item_entity) else {
        return;
    };

    commands
        .entity(item_entity)
        .observe(usable_with_targeted_effect_used_target_selected);

    let targeting_system = targeting_systems.system(targeting);
    commands.run_system_with(targeting_system, item_entity);
}

fn usable_with_targeted_effect_used_target_selected(
    trigger: Trigger<TargetSelected>,
    q_owner: Query<&ItemOf>,
    q_effect: Query<&UsableWithTargetedEffect>,
    mut commands: Commands,
) {
    let owner = q_owner.get(trigger.source).unwrap().owner();
    let effect = q_effect.get(trigger.source).unwrap();

    // Call the effect function stored in the component
    let ctx = EffectContext {
        owner,
        target: trigger.target,
        source: trigger.source,
    };
    (effect.effect_fn)(ctx, &mut commands);

    commands.entity(trigger.observer()).despawn();
}
