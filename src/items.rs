use crate::fighting::{AttackEvent, Battle, TickEvent};
use bevy::prelude::*;
use std::time::Duration;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(tick_usable);
        app.add_observer(weapon_used);
    }
}

fn tick_usable(
    _: Trigger<TickEvent>,
    mut q_usable: Query<(&mut Usable, Entity)>,
    mut commands: Commands,
) {
    q_usable.iter_mut().for_each(|(mut usable, entity)| {
        usable.current_tick = usable.current_tick.saturating_sub(1);
        if usable.current_tick == 0 {
            commands.trigger_targets(UseEvent {}, entity);
            usable.current_tick = usable.max_ticks;
        }
    });
}

fn weapon_used(
    trigger: Trigger<UseEvent>,
    query: Query<(&Weapon, &Parent), With<Usable>>,
    mut commands: Commands,
    battle: Res<Battle>,
) {
    let id = trigger.entity();
    let Ok((weapon, parent)) = query.get(id) else {
        return;
    };

    let user = parent.get();
    let other = battle.opponent(user);
    commands.trigger_targets(
        AttackEvent {
            attacker: user,
            defender: other,
            attacked_with: id,
        },
        other,
    );
}

#[derive(Component)]
pub struct Item;

#[derive(Component)]
pub struct Usable {
    max_ticks: u32,
    current_tick: u32,
}

impl Usable {
    pub fn new(max_ticks: u32) -> Self {
        Self {
            max_ticks,
            current_tick: max_ticks,
        }
    }

    pub fn with_cooldown(seconds: f64) -> Self {
        Self::new((seconds * 10.0) as u32)
    }
}

#[derive(Event)]
pub struct UseEvent;

#[derive(Component)]
pub struct Weapon {
    pub damage: u32,
}

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
            usable: Usable::with_cooldown(3.0),
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
            usable: Usable::with_cooldown(cooldown.as_secs_f64()),
            weapon: Weapon { damage },
        }
    }
}
