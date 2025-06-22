use crate::characters::{Character, ItemOf, Items};
use crate::fighting::Battle;
use bevy::ecs::system::SystemId;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use rand::prelude::*;

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

#[derive(Event)]
pub struct TargetSelected {
    pub source: Entity,
    pub target: Entity,
}
pub fn random_opponent_item(
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

    // Now we need to find a random item in the opponent's inventory
    let Ok(opponent_items) = q_opponent.get(opponent_entity) else {
        return;
    };

    let available_items = opponent_items.iter();

    // Pick a random item from the available items
    let mut rng = thread_rng();
    let Some(target) = available_items.choose(&mut rng) else {
        return;
    };

    commands.trigger_targets(TargetSelected { source, target }, source);
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
    commands.trigger_targets(TargetSelected { source, target }, source);
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

    commands.trigger_targets(TargetSelected { source, target }, source);
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
        commands.trigger_targets(TargetSelected { source, target }, source);
    }
}