use crate::characters::{Character, Health};
use crate::fighting::Battle;
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct Poison(u32);

impl Poison {
    pub fn new(amount: u32) -> Self {
        Poison(amount)
    }
}

#[derive(Event)]
pub struct PoisonEvent {
    attacker: Entity,
    defender: Entity,
    with: Entity,
}

impl PoisonEvent {
    pub fn new(attacker: Entity, defender: Entity, with: Entity) -> Self {
        Self {
            attacker,
            defender,
            with,
        }
    }
}

#[derive(Component)]
pub struct Poisoned {
    amount: u32,
    timer: Timer,
}

impl Poisoned {
    pub fn new(amount: u32) -> Self {
        Self {
            amount,
            timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
        }
    }
}

pub fn tick_poisoned(
    time: Res<Time>,
    battle: Res<Battle>,
    mut q_poisoned: Query<(Entity, &Name, &mut Poisoned, &mut Health)>,
    mut commands: Commands,
) {
    q_poisoned
        .iter_mut()
        .for_each(|(entity, name, mut poisoned, mut health)| {
            poisoned.timer.tick(time.delta());
            
            if !poisoned.timer.just_finished() {
                return;
            }

            let poison_amt = poisoned.amount;
            if poison_amt == 0 {
                commands.entity(entity).remove::<Poisoned>();
                return;
            }

            eprintln!(
                "{:?}: Poisoned {:?} for {}",
                battle.elapsed(time.elapsed_secs_f64()),
                name,
                poison_amt,
            );

            // Poison directly affects health, bypassing shields
            health.current = health.current.saturating_sub(poison_amt);
        });
}

pub fn on_poisoned(
    trigger: Trigger<PoisonEvent>,
    time: Res<Time>,
    battle: Res<Battle>,
    q_attacker: Query<&Name>,
    mut q_defender: Query<(&Name, Option<&mut Poisoned>), With<Character>>,
    q_poisoner: Query<(&Poison, &Name)>,
    mut commands: Commands,
) {
    let PoisonEvent {
        attacker,
        defender,
        with,
    } = trigger.event();
    let (defender_name, maybe_poisoned) = q_defender
        .get_mut(*defender)
        .expect("defender should exist");
    let attacker_name = q_attacker.get(*attacker).expect("attacker should exist");

    let (poison, source_name) = q_poisoner.get(*with).expect("poison source should exist");

    if let Some(mut poisoned) = maybe_poisoned {
        poisoned.amount += poison.0;
    } else {
        // we need to add a new poisoned component to the defender
        commands.entity(*defender).insert(Poisoned::new(poison.0));
    }

    eprintln!(
        "{:?}: {:?} poisoned {:?} with {} for {}!",
        battle.elapsed(time.elapsed_secs_f64()),
        attacker_name,
        defender_name,
        source_name,
        poison.0,
    );
}
