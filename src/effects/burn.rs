use crate::characters::{Character, Health};
use crate::effects::shield::Shielded;
use crate::fighting::{Battle, MajorTickEvent};
use bevy::prelude::*;

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

#[derive(Default, Component)]
pub struct Burning(u32);

pub fn tick_burning(
    _: Trigger<MajorTickEvent>,
    time: Res<Time>,
    battle: Res<Battle>,
    mut q_burn: Query<(&Name, &mut Burning, &mut Health, &mut Shielded)>,
) {
    q_burn
        .iter_mut()
        .for_each(|(name, mut burning, mut health, mut shielded)| {
            let burn_amt = burning.0;
            if burn_amt == 0 {
                return;
            }

            eprintln!(
                "{:?}: Burning {:?} for {}",
                battle.elapsed(time.elapsed_secs_f64()),
                name,
                burn_amt,
            );
            // burn shields
            let burn_amt = shielded.absorb(burn_amt);
            if burn_amt == 0 {
                return;
            }
            // then health
            health.current = health.current.saturating_sub(burn_amt);

            // and remove one burn
            burning.0 = burning.0.saturating_sub(1);
        });
}

pub fn on_burned(
    trigger: Trigger<BurnEvent>,
    time: Res<Time>,
    battle: Res<Battle>,
    q_attacker: Query<&Name>,
    mut q_defender: Query<(&mut Burning, &Name), With<Character>>,
    q_burner: Query<(&Burn, &Name)>,
) {
    let BurnEvent {
        attacker,
        defender,
        with,
    } = trigger.event();
    let (mut burned, defender_name) = q_defender.get_mut(*defender).expect("defender not found");
    let attacker_name = q_attacker.get(*attacker).expect("attacker not found");

    let (burn, source_name) = q_burner.get(*with).expect("burn source not found");
    burned.0 += burn.0;

    eprintln!(
        "{:?}: {:?} burned {:?} with {} for {}!",
        battle.elapsed(time.elapsed_secs_f64()),
        attacker_name,
        defender_name,
        source_name,
        burn.0,
    );
}
