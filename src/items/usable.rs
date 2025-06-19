use crate::effects::freeze::Frozen;
use crate::effects::haste::Hastened;
use crate::effects::slow::Slowed;
use crate::fighting::{Battle, TickEvent};
use bevy::prelude::*;
use std::time::Duration;

#[allow(clippy::type_complexity)]
pub fn tick_usable(
    _: Trigger<TickEvent>,
    battle: Res<Battle>,
    mut query: Query<(
        &mut Usable,
        Entity,
        &Name,
        Option<&mut Hastened>,
        Option<&mut Slowed>,
        Option<&mut Frozen>,
    )>,
    mut commands: Commands,
) {
    // Process all items and their conditions in a single loop
    for (mut usable, entity, name, mut maybe_hastened, mut maybe_slowed, mut maybe_frozen) in
        query.iter_mut()
    {
        // First, tick condition timers if they exist
        if let Some(ref mut frozen) = maybe_frozen {
            frozen.timer.tick(Duration::from_millis(100));
            if frozen.timer.just_finished() {
                info!(
                    "{:?}: Frozen condition expired on {:?}",
                    battle.elapsed, name
                );
                commands.entity(entity).remove::<Frozen>();
            }
        }

        if let Some(ref mut hastened) = maybe_hastened {
            hastened.timer.tick(Duration::from_millis(100));
            if hastened.timer.just_finished() {
                info!(
                    "{:?}: Hastened condition expired on {:?}",
                    battle.elapsed, name
                );
                commands.entity(entity).remove::<Hastened>();
            }
        }

        if let Some(ref mut slowed) = maybe_slowed {
            slowed.timer.tick(Duration::from_millis(100));
            if slowed.timer.just_finished() {
                info!(
                    "{:?}: Slowed condition expired on {:?}",
                    battle.elapsed, name
                );
                commands.entity(entity).remove::<Slowed>();
            }
        }

        // Process usable item cooldown
        let timer = &mut usable.cooldown;

        // Determine which conditions are active
        let is_frozen = maybe_frozen.is_some();
        let is_hastened = maybe_hastened.is_some();
        let is_slowed = maybe_slowed.is_some();

        // Apply the appropriate tick duration based on conditions
        if is_frozen {
            // Frozen items don't tick their cooldown
            // Do nothing here
        } else if is_hastened && is_slowed {
            // If both hastened and slowed, they cancel out
            timer.tick(Duration::from_millis(100));
        } else if is_hastened {
            // Hastened items tick at double speed (200ms instead of 100ms)
            timer.tick(Duration::from_millis(200));
        } else if is_slowed {
            // Slowed items tick at half speed (50ms instead of 100ms)
            timer.tick(Duration::from_millis(50));
        } else {
            // Normal tick speed
            timer.tick(Duration::from_millis(100));
        }

        if timer.just_finished() {
            info!("{:?}: used {:?}!", battle.elapsed, name);

            commands.trigger_targets(UseEvent {}, entity);
            // reset the cooldown
            usable.cooldown = Timer::new(timer.duration(), TimerMode::Once);
        }
    }
}

#[derive(Component)]
pub struct Usable {
    cooldown: Timer,
}

impl Usable {
    pub fn with_cooldown(duration: Duration) -> Self {
        Self {
            cooldown: Timer::new(duration, TimerMode::Once),
        }
    }
}

#[derive(Event)]
pub struct UseEvent;
