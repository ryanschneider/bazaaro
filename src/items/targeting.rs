use crate::characters::{Character, Hero, ItemOf, Items, Villain};
use crate::fighting::{BazaaroCharacter, BazaaroRng, Battle, RngKey};
use crate::rng::RngProvider;
use bevy::ecs::system::SystemId;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;

#[derive(Component, Eq, Hash, PartialEq, Default)]
pub enum Targeting {
    #[default]
    RandomOpponentItem,
    LeftmostDifferentItem,
    RightmostOpponentItem,
    AllOpponentItems,
}

pub(crate) fn targeting_startup(mut commands: Commands) {
    let mut systems = TargetingSystems::default();
    systems.0.insert(
        Targeting::RandomOpponentItem,
        commands.register_system(random_opponent_item),
    );
    systems.0.insert(
        Targeting::LeftmostDifferentItem,
        commands.register_system(leftmost_different_item),
    );
    systems.0.insert(
        Targeting::RightmostOpponentItem,
        commands.register_system(rightmost_opponent_item),
    );
    systems.0.insert(
        Targeting::AllOpponentItems,
        commands.register_system(all_opponent_items),
    );
    commands.insert_resource(systems);
}

#[derive(Resource, Default)]
pub struct TargetingSystems(HashMap<Targeting, SystemId<In<Entity>>>);

impl TargetingSystems {
    pub fn system(&self, from: &Targeting) -> SystemId<In<Entity>> {
        match from {
            Targeting::RandomOpponentItem => *self.0.get(&Targeting::RandomOpponentItem).unwrap(),
            Targeting::LeftmostDifferentItem => {
                *self.0.get(&Targeting::LeftmostDifferentItem).unwrap()
            },
            Targeting::RightmostOpponentItem => {
                *self.0.get(&Targeting::RightmostOpponentItem).unwrap()
            },
            Targeting::AllOpponentItems => {
                *self.0.get(&Targeting::AllOpponentItems).unwrap()
            }
        }
    }
}

#[derive(EntityEvent)]
pub struct TargetSelected {
    pub source: Entity,
    #[event_target]
    pub target: Entity,
}

/// Helper function to determine which character (Hero/Villain) owns an entity
fn get_character_type(
    entity: Entity,
    q_hero: &Query<(), With<Hero>>,
    q_villain: &Query<(), With<Villain>>,
) -> Option<BazaaroCharacter> {
    if q_hero.get(entity).is_ok() {
        Some(BazaaroCharacter::Hero)
    } else if q_villain.get(entity).is_ok() {
        Some(BazaaroCharacter::Villain)
    } else {
        None
    }
}
pub fn random_opponent_item(
    In(source): In<Entity>,
    battle: Res<Battle>,
    mut rng: ResMut<BazaaroRng>,
    q_owner: Query<&ItemOf>,
    q_items: Query<&Items, With<Character>>,
    q_hero: Query<(), With<Hero>>,
    q_villain: Query<(), With<Villain>>,
    mut commands: Commands,
) {
    let Ok(item_of) = q_owner.get(source.entity()) else {
        return;
    };
    let owner = item_of.owner();

    // Determine character type of owner
    let Some(character) = get_character_type(owner, &q_hero, &q_villain) else {
        return;
    };

    // Find the index of the source item in the owner's inventory
    let Ok(owner_items) = q_items.get(owner) else {
        return;
    };
    let Some(source_index) = owner_items.iter().position(|item| item == source) else {
        return;
    };

    // Find the opponent using the battle resource
    let opponent_entity = battle.opponent(owner);

    // Now we need to find a random item in the opponent's inventory
    let Ok(opponent_items) = q_items.get(opponent_entity) else {
        return;
    };

    let available_items: Vec<Entity> = opponent_items.iter().collect();
    if available_items.is_empty() {
        return;
    }

    // Build RNG key with stable identifiers
    let key = RngKey {
        tick: battle.tick,
        character,
        index: source_index,
    };

    // Pick a random item using deterministic RNG
    let target_idx = rng.result_range(key, 0..available_items.len() as u32);
    let target = available_items[target_idx as usize];

    commands.entity(target).trigger(|target| TargetSelected { source, target });
}

pub fn leftmost_different_item(
    In(source): In<Entity>,
    q_owner: Query<&ItemOf>,
    q_owners_items: Query<&Items, With<Character>>,
    mut commands: Commands,
) {
    let Ok(item_of) = q_owner.get(source.entity()) else {
        return;
    };
    let owner = item_of.owner();
    let Ok(owner_items) = q_owners_items.get(owner) else {
        return;
    };
    let target = owner_items.iter().find(|item| *item != source);
    let Some(target) = target else {
        return;
    };
    commands.entity(target).trigger(|target| TargetSelected { source, target });
}

pub fn rightmost_opponent_item(
    In(source): In<Entity>,
    battle: Res<Battle>,
    q_owner: Query<&ItemOf>,
    q_opponent: Query<&Items, With<Character>>,
    mut commands: Commands,
) {
    let Ok(item_of) = q_owner.get(source.entity()) else {
        return;
    };
    let owner = item_of.owner();
    // Find the opponent using the battle resource
    let opponent_entity = battle.opponent(owner);

    // Now we need to find the last (rightmost) item in the opponents inventory
    let Ok(opponent_items) = q_opponent.get(opponent_entity) else {
        return;
    };
    let available_items = opponent_items.iter();
    let Some(target) = available_items.last() else {
        return;
    };

    commands.entity(target).trigger(|target| TargetSelected { source, target });
}

pub fn all_opponent_items(
    In(source): In<Entity>,
    battle: Res<Battle>,
    q_owner: Query<&ItemOf>,
    q_opponent: Query<&Items, With<Character>>,
    mut commands: Commands,
) {
    let Ok(item_of) = q_owner.get(source.entity()) else {
        return;
    };
    let owner = item_of.owner();
    // Find the opponent using the battle resource
    let opponent_entity = battle.opponent(owner);

    // Now we need to find the last (rightmost) item in the opponents inventory
    let Ok(opponent_items) = q_opponent.get(opponent_entity) else {
        return;
    };
    let available_items = opponent_items.iter();
    for target in available_items {
        commands.entity(target).trigger(|target| TargetSelected { source, target });
    }
}