use bevy::prelude::*;
use std::time::Duration;

use crate::fighting::Battle;
use crate::items::Item;

#[derive(Component)]
pub struct Slow {
    pub duration: Duration,
}

impl Slow {
    pub fn new(duration_secs: f32) -> Self {
        Self {
            duration: Duration::from_secs_f32(duration_secs),
        }
    }
}

#[derive(Event)]
pub struct SlowEvent {
    pub source: Entity,
    pub target: Entity,
    pub with: Entity,
}

impl SlowEvent {
    pub fn new(source: Entity, target: Entity, with: Entity) -> Self {
        Self {
            source,
            target,
            with,
        }
    }
}

#[derive(Component)]
pub struct Slowed {
    pub timer: Timer,
}

impl Slowed {
    pub fn new(duration: Duration) -> Self {
        Self {
            timer: Timer::new(duration, TimerMode::Once),
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn on_slowed(
    trigger: Trigger<SlowEvent>,
    battle: Res<Battle>,
    q_source: Query<&Name>,
    q_target: Query<&Name, With<Item>>,
    q_slow: Query<(&Slow, &Name)>,
    mut q_slowed_items: Query<&mut Slowed>,
    mut commands: Commands,
) {
    let SlowEvent {
        source,
        target,
        with,
    } = trigger.event();

    let source_name = q_source.get(*source).expect("source should exist");
    let target_name = q_target.get(*target).expect("target should exist");

    let (slow, item_name) = q_slow.get(*with).expect("slow source should exist");

    // If the target already has the Slowed component, extend its duration
    match q_slowed_items.get_mut(*target) {
        Ok(mut slowed) => {
            let remaining = slowed.timer.remaining();
            slowed.timer.set_duration(remaining + slow.duration);
            slowed.timer.reset();

            info!(
                "{:?}: {:?} applied additional slow to {:?} with {:?}! Total duration: {:?}",
                battle.elapsed,
                source_name,
                target_name,
                item_name,
                slowed.timer.duration(),
            );
        }
        Err(_) => {
            // Add a new Slowed component to the target item
            commands.entity(*target).insert(Slowed::new(slow.duration));

            info!(
                "{:?}: {:?} slowed {:?} with {:?} for {:?}!",
                battle.elapsed, source_name, target_name, item_name, slow.duration,
            );
        }
    }
}
