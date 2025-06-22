use crate::effects::freeze::Freeze;
use crate::effects::haste::Haste;
use crate::effects::heal::Heal;
use crate::effects::poison::Poison;
use crate::effects::slow::Slow;
use crate::items::freezer::freezer;
use crate::items::hastener::hastener;
use crate::items::slower::Slower;
use crate::items::targeting::Targeting;
use crate::items::usable::Usable;
use crate::items::weapons::Weapon;
use crate::items::Item;
use bevy::prelude::*;
use std::time::Duration;
use crate::items::usable_with_targeted_effect::UsableWithTargetedEffect;

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

#[derive(Bundle)]
pub struct HealingPotion {
    pub name: Name,
    pub item: Item,
    pub usable: Usable,
    pub heal: Heal,
}

impl Default for HealingPotion {
    fn default() -> Self {
        Self {
            name: "Healing Potion".into(),
            item: Item,
            usable: Usable::with_cooldown(Duration::from_secs(5)),
            heal: Heal::new(20),
        }
    }
}

#[derive(Bundle)]
pub struct GorgonsHead {
    pub name: Name,
    pub item: Item,
    pub usable: Usable,
    pub slow: Slow,
    pub slower: Slower,
}

impl Default for GorgonsHead {
    fn default() -> Self {
        Self {
            name: "Gorgon's Head".into(),
            item: Item,
            usable: Usable::with_cooldown(Duration::from_secs(2)),
            slow: Slow::new(1.0), // Slows for 1 second
            slower: Slower,
        }
    }
}

#[derive(Bundle)]
pub struct FreezingCrystal {
    pub name: Name,
    pub item: Item,
    pub usable: Usable,
    pub freeze: Freeze,
    pub targeting: Targeting,
    pub freezer: UsableWithTargetedEffect
}

impl Default for FreezingCrystal {
    fn default() -> Self {
        Self {
            name: "Freezing Crystal".into(),
            item: Item,
            usable: Usable::with_cooldown(Duration::from_secs(3)),
            freeze: Freeze::new(1.5), // Freezes for 1.5 seconds
            targeting: Targeting::RandomOpponentItem,
            freezer: freezer(),
        }
    }
}

#[derive(Bundle)]
pub struct HastePotion {
    pub name: Name,
    pub item: Item,
    pub usable: Usable,
    pub haste: Haste,
    pub targeting: Targeting,
    pub hastener: UsableWithTargetedEffect,
}

impl Default for HastePotion {
    fn default() -> Self {
        Self {
            name: "Haste Potion".into(),
            item: Item,
            usable: Usable::with_cooldown(Duration::from_secs(4)),
            haste: Haste::new(2.0), // Hastens for 2 seconds
            targeting: Targeting::LeftmostDifferentItem,
            hastener: hastener(),
        }
    }
}
