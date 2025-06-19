use crate::characters::{Character, Health};
use crate::effects::shield::Shielded;
use crate::fighting::Battle;
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct Burn(u32);

impl Burn {
    pub fn new(amount: u32) -> Self {
        Burn(amount)
    }
}

#[derive(Event)]
pub struct BurnEvent {
    attacker: Entity,
    defender: Entity,
    with: Entity,
}

impl BurnEvent {
    pub fn new(attacker: Entity, defender: Entity, with: Entity) -> Self {
        Self {
            attacker,
            defender,
            with,
        }
    }
}

#[derive(Component)]
pub struct Burning {
    amount: u32,
    timer: Timer,
}

impl Burning {
    pub fn new(amount: u32) -> Self {
        Self {
            amount,
            timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
        }
    }
}

pub fn tick_burning(
    time: Res<Time>,
    battle: Res<Battle>,
    mut q_burn: Query<(
        Entity,
        &Name,
        &mut Burning,
        &mut Health,
        Option<&mut Shielded>,
    )>,
    mut commands: Commands,
) {
    q_burn
        .iter_mut()
        .for_each(|(entity, name, mut burning, mut health, shielded)| {
            burning.timer.tick(time.delta());
            if !burning.timer.just_finished() {
                return;
            }

            let burn_amt = burning.amount;
            if burn_amt == 0 {
                commands.entity(entity).remove::<Burning>();
                return;
            }

            info!("{:?}: Burning {:?} for {}", battle.elapsed, name, burn_amt,);

            // if they have shields, burn that
            // not that it appears that any shield will block all burn
            // so this does not fall through.
            match shielded {
                Some(mut shielded) if shielded.0 > 0 => {
                    let original_shielded = shielded.0;
                    shielded.0 = shielded.0.saturating_sub(burn_amt / 2);
                    info!(
                        "{:?}: {} shield blocked {} burn, {} shield remains!",
                        battle.elapsed, original_shielded, burn_amt, shielded.0,
                    );
                    if shielded.0 == 0 {
                        commands.entity(entity).remove::<Shielded>();
                    }
                }
                _ => {
                    // otherwise burn health
                    health.current = health.current.saturating_sub(burn_amt);
                }
            };

            // and decrement one burn
            burning.amount = burning.amount.saturating_sub(1);
            if burning.amount == 0 {
                commands.entity(entity).remove::<Burning>();
            }
        });
}

pub fn on_burned(
    trigger: Trigger<BurnEvent>,
    battle: Res<Battle>,
    q_attacker: Query<&Name>,
    mut q_defender: Query<(&Name, Option<&mut Burning>), With<Character>>,
    q_burner: Query<(&Burn, &Name)>,
    mut commands: Commands,
) {
    let BurnEvent {
        attacker,
        defender,
        with,
    } = trigger.event();
    let (defender_name, maybe_burning) = q_defender
        .get_mut(*defender)
        .expect("defender should exist");
    let attacker_name = q_attacker.get(*attacker).expect("attacker should exist");

    let (burn, source_name) = q_burner.get(*with).expect("burn source should exist");

    if let Some(mut burning) = maybe_burning {
        burning.amount += burn.0;
    } else {
        // we need to add a new burning component to the defender
        commands.entity(*defender).insert(Burning::new(burn.0));
    }

    info!(
        "{:?}: {:?} burned {:?} with {} for {}!",
        battle.elapsed, attacker_name, defender_name, source_name, burn.0,
    );
}
