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

pub fn on_attack(
    trigger: Trigger<AttackEvent>,
    time: Res<Time>,
    battle: Res<Battle>,
    q_attacker: Query<&Name>,
    mut q_defender: Query<(&mut Health, &mut Shielded, Option<&Name>), With<Character>>,
    q_weapon: Query<(&Weapon, Option<&Name>)>,
) {
    let AttackEvent {
        attacker,
        defender,
        with,
    } = trigger.event();
    let Ok((mut health, mut shielded, defender_name)) = q_defender.get_mut(*defender) else {
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
    let damage = shielded.absorb(damage);
    health.current = health.current.saturating_sub(damage);
    let weapon_name: &str = weapon_name.map_or("some weapon", |n| n.as_str());

    eprintln!(
        "{:?}: {:?} attacked {:?} with {} for {}!",
        battle.elapsed(time.elapsed_secs_f64()),
        attacker_name,
        defender_name,
        weapon_name,
        damage
    );
}
