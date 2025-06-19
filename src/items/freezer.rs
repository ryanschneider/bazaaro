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
    battle: Res<Battle>,
    q_character: Query<(Entity, &Items), With<Character>>,
    q_freezer: Query<(Entity, &ItemOf), With<Freezer>>,
    mut commands: Commands,
) {
    // The entity that triggered the event
    let freezer_entity = trigger.target();

    // Only continue if the item has the Freezer component
    let (freezer_entity, item_of) = match q_freezer.get(freezer_entity) {
        Ok(result) => result,
        Err(_) => return,
    };

    // Find the owner of this item
    let source_entity = item_of.owner();

    // Find the opponent using the battle resource
    let opponent_entity = battle.opponent(source_entity);

    // Now we need to find a random item in the opponent's inventory
    let (_, opponent_items) = match q_character.get(opponent_entity) {
        Ok(result) => result,
        Err(_) => return,
    };

    // Collect all non-empty slots
    let available_items = opponent_items.iter();
    
    // Pick a random item from the available items
    let mut rng = thread_rng();
    let Some(target_item) = available_items.choose(&mut rng) else {
        return;
    };

    // Trigger the freeze event
    commands.trigger(FreezeEvent::new(source_entity, target_item, freezer_entity));
}
