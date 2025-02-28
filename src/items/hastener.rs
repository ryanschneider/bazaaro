use bevy::prelude::*;

use crate::characters::{Character, Items};
use crate::effects::haste::HasteEvent;
use crate::items::usable::UseEvent;

#[derive(Component)]
pub struct Hastener;

pub fn hastener_used(
    trigger: Trigger<UseEvent>,
    q_character: Query<(Entity, &Items), With<Character>>,
    q_hastener: Query<(Entity, &Parent), With<Hastener>>,
    mut commands: Commands,
) {
    // The entity that triggered the event
    let hastener_entity = trigger.entity();

    // Only continue if the item has the Hastener component
    let (hastener_entity, parent) = match q_hastener.get(hastener_entity) {
        Ok(result) => result,
        Err(_) => return,
    };

    // Find the owner of this item
    let source_entity = parent.get();

    // The Haste Potion targets a friendly item (our own item)
    // We'll pick the leftmost item (which is usually a weapon) to make it more useful
    let (_, owner_items) = match q_character.get(source_entity) {
        Ok(result) => result,
        Err(_) => return,
    };

    // Get the leftmost non-empty slot (but not the haste potion itself)
    let target_item = owner_items
        .slots
        .iter()
        .filter_map(|slot| slot.as_ref().copied())
        .filter(|item| *item != hastener_entity) // Don't target self
        .next();

    // If we found a target item, trigger the haste event
    if let Some(target_item) = target_item {
        commands.trigger(HasteEvent::new(source_entity, target_item, hastener_entity));
    }
}