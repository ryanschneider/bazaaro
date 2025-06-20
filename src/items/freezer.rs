use bevy::prelude::*;
use rand::prelude::*;

use crate::characters::{Character, ItemOf, Items};
use crate::effects::freeze::FreezeEvent;
use crate::fighting::Battle;
use crate::items::usable::UseEvent;

#[derive(Component)]
pub struct Freezer;

pub fn freezer_used(
    trigger: Trigger<UseEvent>,
    q_freezer: Query<Entity, (With<Freezer>, With<ItemOf>)>,
    mut commands: Commands,
) {
    // The entity that triggered the event
    let freezer_entity = trigger.target();

    // Only continue if the item has the Freezer component
    let Ok(freezer_entity) = q_freezer.get(freezer_entity) else {
        return;
    };

    // TODO: add a required Targeting component that tells us which targeting system to use,
    // then call that using this approach.
    let system_id = commands.register_system(random_opponent_item.pipe(freezer_used_callback));
    commands.run_system_with(system_id, freezer_entity);
}

fn freezer_used_callback(
    In(result): In<Result<Option<Target>>>,
    q_owner: Query<&ItemOf>,
    mut commands: Commands,
) {
    let Ok(Some(target)) = result else {
        return;
    };
    let owner = q_owner.get(target.source).unwrap().owner();
    commands.trigger(FreezeEvent::new(owner, target.target, target.source));
}

pub struct Target {
    source: Entity,
    target: Entity,
}

pub fn random_opponent_item(
    In(source): In<Entity>,
    battle: Res<Battle>,
    q_owner: Query<&ItemOf>,
    q_opponent: Query<(Entity, &Items), With<Character>>,
) -> Result<Option<Target>> {
    let Ok(item_of) = q_owner.get(source.entity()) else {
        return Ok(None);
    };
    let owner = item_of.owner();
    // Find the opponent using the battle resource
    let opponent_entity = battle.opponent(owner);

    // Now we need to find a random item in the opponent's inventory
    let (_, opponent_items) = q_opponent.get(opponent_entity)?;

    // Collect all non-empty slots
    let available_items = opponent_items.iter();

    // Pick a random item from the available items
    let mut rng = thread_rng();
    let Some(target) = available_items.choose(&mut rng) else {
        return Ok(None);
    };

    Ok(Some(Target { source, target }))
}
