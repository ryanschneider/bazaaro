use crate::characters::Character;
use crate::fighting::Battle;
use bevy::core::Name;
use bevy::prelude::*;

#[derive(Component)]
pub struct Shield(u32);

impl Shield {
    pub fn new(amount: u32) -> Self {
        Shield(amount)
    }
}

#[derive(Event)]
pub struct ShieldEvent {
    defender: Entity,
    with: Entity,
}

impl ShieldEvent {
    pub fn new(defender: Entity, with: Entity) -> Self {
        Self { defender, with }
    }
}

#[derive(Default, Component)]
pub struct Shielded(pub u32);

impl Shielded {
    pub fn absorb(&mut self, dmg: u32) -> u32 {
        if dmg > self.0 {
            let remaining = dmg - self.0;
            self.0 = 0;
            remaining
        } else {
            0
        }
    }
}

pub fn on_shield(
    trigger: Trigger<ShieldEvent>,
    battle: Res<Battle>,
    mut q_defender: Query<(&Name, Option<&mut Shielded>), With<Character>>,
    q_with: Query<(&Name, &Shield)>,
    mut commands: Commands,
) {
    let ShieldEvent { defender, with } = trigger.event();

    let (defender_name, maybe_shielded) = q_defender.get_mut(*defender).expect("defender exists");
    let (source_name, shield) = q_with.get(*with).expect("source exists");

    if let Some(mut shielded) = maybe_shielded {
        shielded.0 += shield.0;
    } else {
        commands.entity(*defender).insert(Shielded(shield.0));
    }

    eprintln!(
        "{:?}: {:?} shielded with {} for {}!",
        battle.elapsed, defender_name, source_name, shield.0,
    );
}
