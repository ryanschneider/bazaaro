use crate::effects::attack::AttackEvent;
use crate::fighting::Battle;
use crate::items::usable::UseEvent;
use bevy::prelude::*;

#[derive(Component)]
pub struct Weapon {
    pub damage: u32,
}

pub fn weapon_used(
    trigger: Trigger<UseEvent>,
    query: Query<&ChildOf, With<Weapon>>,
    mut commands: Commands,
    battle: Res<Battle>,
) {
    let attacked_with = trigger.target();
    let Ok(child_of) = query.get(attacked_with) else {
        return;
    };

    let attacker = child_of.parent();
    let defender = battle.opponent(attacker);
    commands.trigger_targets(
        AttackEvent::new(attacker, defender, attacked_with),
        defender,
    );
}
