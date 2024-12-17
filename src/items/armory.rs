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
