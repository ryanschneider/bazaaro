use crate::characters::{Character, Health};
use crate::effects::poison::Poisoned;
use crate::fighting::Battle;
use bevy::prelude::*;

#[derive(Component)]
pub struct Heal(pub u32);

impl Heal {
    pub fn new(amount: u32) -> Self {
        Heal(amount)
    }
}

#[derive(Event)]
pub struct HealEvent {
    target: Entity,
    with: Entity,
}

impl HealEvent {
    pub fn new(target: Entity, with: Entity) -> Self {
        Self { target, with }
    }
}

pub fn on_heal(
    trigger: Trigger<HealEvent>,
    battle: Res<Battle>,
    mut q_target: Query<(Entity, &Name, &mut Health, Option<&mut Poisoned>), With<Character>>,
    q_heal_source: Query<(&Heal, &Name)>,
    mut commands: Commands,
) {
    let HealEvent { target, with } = trigger.event();

    let Ok((entity, target_name, mut health, maybe_poisoned)) = q_target.get_mut(*target) else {
        return;
    };

    let Ok((heal, source_name)) = q_heal_source.get(*with) else {
        return;
    };

    let heal_amount = heal.0;

    // Calculate how much we can actually heal
    let healing_needed = health.max.saturating_sub(health.current);
    let actual_healing = heal_amount.min(healing_needed);

    if actual_healing > 0 {
        health.current += actual_healing;

        eprintln!(
            "{:?}: {:?} healed for {} using {}!",
            battle.elapsed, target_name, actual_healing, source_name,
        );
    }

    // If the character is poisoned, remove 1 point of poison when healed
    if let Some(mut poisoned) = maybe_poisoned {
        if poisoned.amount > 0 {
            poisoned.amount = poisoned.amount.saturating_sub(1);

            eprintln!(
                "{:?}: Healing reduced poison by 1 for {:?}!",
                battle.elapsed, target_name,
            );

            // If there's no more poison, remove the Poisoned component
            if poisoned.amount == 0 {
                commands.entity(entity).remove::<Poisoned>();

                eprintln!(
                    "{:?}: {:?} is no longer poisoned!",
                    battle.elapsed, target_name,
                );
            }
        }
    }
}
