use crate::effects::poison::Poison;
use crate::items::usable::Usable;
use crate::items::weapons::Weapon;
use crate::items::Item;
use bevy::core::Name;
use bevy::prelude::Bundle;
use std::time::Duration;

#[derive(Bundle)]
pub struct HandAxe {
    pub name: Name,
    pub item: Item,
    pub usable: Usable,
    pub weapon: Weapon,
}

impl Default for HandAxe {
    fn default() -> Self {
        Self {
            name: "Hand Axe".into(),
            item: Item,
            usable: Usable::with_cooldown(Duration::from_secs(3)),
            weapon: Weapon { damage: 5 },
        }
    }
}

#[derive(Bundle)]
pub struct GenericWeapon {
    pub name: Name,
    pub item: Item,
    pub usable: Usable,
    pub weapon: Weapon,
}

impl GenericWeapon {
    pub fn new(name: &str, damage: u32, cooldown: Duration) -> Self {
        Self {
            name: name.into(),
            item: Item,
            usable: Usable::with_cooldown(cooldown),
            weapon: Weapon { damage },
        }
    }
}

#[derive(Bundle)]
pub struct GenericUsable {
    pub name: Name,
    pub item: Item,
    pub usable: Usable,
}

impl GenericUsable {
    pub fn new(name: &str, cooldown: Duration) -> Self {
        Self {
            name: name.into(),
            item: Item,
            usable: Usable::with_cooldown(cooldown),
        }
    }
}

#[derive(Bundle)]
pub struct PoisonedDagger {
    pub name: Name,
    pub item: Item,
    pub usable: Usable,
    pub weapon: Weapon,
    pub poison: Poison,
}

impl Default for PoisonedDagger {
    fn default() -> Self {
        Self {
            name: "Poisoned Dagger".into(),
            item: Item,
            usable: Usable::with_cooldown(Duration::from_secs(4)),
            weapon: Weapon { damage: 3 },
            poison: Poison::new(5),
        }
    }
}
