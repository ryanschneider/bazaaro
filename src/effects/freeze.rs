use bevy::prelude::*;
use std::time::Duration;

use crate::fighting::Battle;
use crate::items::Item;

#[derive(Component)]
pub struct Freeze {
    pub duration: Duration,
}

impl Freeze {
    pub fn new(duration_secs: f32) -> Self {
        Self {
            duration: Duration::from_secs_f32(duration_secs),
        }
    }
}

#[derive(Event)]
pub struct FreezeEvent {
    pub source: Entity,
    pub target: Entity,
    pub with: Entity,
}

impl FreezeEvent {
    pub fn new(source: Entity, target: Entity, with: Entity) -> Self {
        Self {
            source,
            target,
            with,
        }
    }
}

#[derive(Component)]
pub struct Frozen {
    pub timer: Timer,
}

impl Frozen {
    pub fn new(duration: Duration) -> Self {
        Self {
            timer: Timer::new(duration, TimerMode::Once),
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn on_frozen(
    trigger: Trigger<FreezeEvent>,
    battle: Res<Battle>,
    q_source: Query<&Name>,
    q_target: Query<&Name, With<Item>>,
    q_freeze: Query<(&Freeze, &Name)>,
    mut q_frozen_items: Query<&mut Frozen>,
    mut commands: Commands,
) {
    let FreezeEvent {
        source,
        target,
        with,
    } = trigger.event();

    let source_name = q_source.get(*source).expect("source should exist");
    let target_name = q_target.get(*target).expect("target should exist");

    let (freeze, item_name) = q_freeze.get(*with).expect("freeze source should exist");

    // If the target already has the Frozen component, extend its duration
    match q_frozen_items.get_mut(*target) {
        Ok(mut frozen) => {
            let remaining = frozen.timer.remaining();
            frozen.timer.set_duration(remaining + freeze.duration);
            frozen.timer.reset();

            info!(
                "{:?}: {:?} applied additional freeze to {} with {}! Total duration: {:?}",
                battle.elapsed,
                source_name,
                target_name,
                item_name,
                frozen.timer.duration(),
            );
        }
        Err(_) => {
            // Add a new Frozen component to the target item
            commands
                .entity(*target)
                .insert(Frozen::new(freeze.duration));

            info!(
                "{:?}: {:?} froze {} with {} for {:?}!",
                battle.elapsed, source_name, target_name, item_name, freeze.duration,
            );
        }
    }
}
