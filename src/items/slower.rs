use bevy::prelude::*;

use crate::characters::{Character, Items};
use crate::effects::slow::SlowEvent;
use crate::fighting::Battle;
use crate::items::usable::UseEvent;

#[derive(Component)]
pub struct Slower;

pub fn slower_used(
    trigger: Trigger<UseEvent>,
    battle: Res<Battle>,
    q_character: Query<(Entity, &Items), With<Character>>,
    q_slower: Query<(Entity, &ChildOf), With<Slower>>,
    mut commands: Commands,
) {
    // The entity that triggered the event
    let slower_entity = trigger.target();

    // Only continue if the item has the Slower component
    let (slower_entity, child_of) = match q_slower.get(slower_entity) {
        Ok(result) => result,
        Err(_) => return,
    };

    // Find the owner of this item
    let source_entity = child_of.parent();

    // Find the opponent using the battle resource
    let opponent_entity = battle.opponent(source_entity);

    // Now we need to find the rightmost item in the opponent's inventory
    let (_, opponent_items) = match q_character.get(opponent_entity) {
        Ok(result) => result,
        Err(_) => return,
    };

    // Get the rightmost non-empty slot
    let target_item = opponent_items
        .slots
        .iter()
        .rev() // Start from the right
        .filter_map(|slot| slot.as_ref().copied())
        .next();

    // If we found a target item, trigger the slow event
    if let Some(target_item) = target_item {
        commands.trigger(SlowEvent::new(source_entity, target_item, slower_entity));
    }
}
