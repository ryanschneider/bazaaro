use bevy::prelude::*;
use rand::prelude::*;

use crate::characters::{Character, Items};
use crate::effects::freeze::FreezeEvent;
use crate::fighting::Battle;
use crate::items::usable::UseEvent;

#[derive(Component)]
pub struct Freezer;

pub fn freezer_used(
    trigger: Trigger<UseEvent>,
    battle: Res<Battle>,
    q_character: Query<(Entity, &Items), With<Character>>,
    q_freezer: Query<(Entity, &ChildOf), With<Freezer>>,
    mut commands: Commands,
) {
    // The entity that triggered the event
    let freezer_entity = trigger.target();

    // Only continue if the item has the Freezer component
    let (freezer_entity, child_of) = match q_freezer.get(freezer_entity) {
        Ok(result) => result,
        Err(_) => return,
    };

    // Find the owner of this item
    let source_entity = child_of.parent();

    // Find the opponent using the battle resource
    let opponent_entity = battle.opponent(source_entity);

    // Now we need to find a random item in the opponent's inventory
    let (_, opponent_items) = match q_character.get(opponent_entity) {
        Ok(result) => result,
        Err(_) => return,
    };

    // Collect all non-empty slots
    let available_items: Vec<Entity> = opponent_items
        .slots
        .iter()
        .filter_map(|slot| slot.as_ref().copied())
        .collect();

    // If there are no items, return early
    if available_items.is_empty() {
        return;
    }

    // Pick a random item from the available items
    let mut rng = thread_rng();
    let random_index = rng.gen_range(0..available_items.len());
    let target_item = available_items[random_index];

    // Trigger the freeze event
    commands.trigger(FreezeEvent::new(source_entity, target_item, freezer_entity));
}
