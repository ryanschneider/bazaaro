use crate::characters::{Character, Health};
use crate::effects::shield::Shielded;
use crate::fighting::Battle;
use crate::items::weapons::Weapon;
use bevy::prelude::*;

#[derive(Event)]
pub struct AttackEvent {
    attacker: Entity,
    defender: Entity,
    with: Entity,
}

impl AttackEvent {
    pub fn new(attacker: Entity, defender: Entity, with: Entity) -> Self {
        Self {
            attacker,
            defender,
            with,
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn on_attack(
    trigger: Trigger<AttackEvent>,
    battle: Res<Battle>,
    q_attacker: Query<&Name>,
    mut q_defender: Query<(&mut Health, Option<&mut Shielded>, Option<&Name>), With<Character>>,
    q_weapon: Query<(&Weapon, &Name)>,
) {
    let AttackEvent {
        attacker,
        defender,
        with,
    } = trigger.event();
    let Ok((mut health, maybe_shielded, defender_name)) = q_defender.get_mut(*defender) else {
        return;
    };
    let defender_name: &str = defender_name
        .map(|name| name.as_str())
        .unwrap_or("defender");
    let attacker_name: &str = q_attacker.get(*attacker).map_or("attacker", |n| n.as_str());

    let Ok((weapon, weapon_name)) = q_weapon.get(*with) else {
        return;
    };
    let damage = weapon.damage;
    let damage = match maybe_shielded {
        None => damage,
        Some(mut shielded) => {
            info!(
                "{:?}: {:?} shield blocked {:?}!",
                battle.elapsed,
                defender_name,
                damage.min(shielded.0)
            );
            shielded.absorb(damage)
        }
    };
    if damage == 0 {
        return;
    }

    health.current = health.current.saturating_sub(damage);
    info!(
        "{:?}: {:?} damaged {:?} with {:?} for {:?}!",
        battle.elapsed, attacker_name, defender_name, weapon_name, damage
    );
}
