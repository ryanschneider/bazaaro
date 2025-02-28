use bevy::prelude::*;
use std::time::Duration;

use crate::fighting::Battle;
use crate::items::Item;

#[derive(Component)]
pub struct Haste {
    pub duration: Duration,
}

impl Haste {
    pub fn new(duration_secs: f32) -> Self {
        Self {
            duration: Duration::from_secs_f32(duration_secs),
        }
    }
}

#[derive(Event)]
pub struct HasteEvent {
    pub source: Entity,
    pub target: Entity,
    pub with: Entity,
}

impl HasteEvent {
    pub fn new(source: Entity, target: Entity, with: Entity) -> Self {
        Self {
            source,
            target,
            with,
        }
    }
}

#[derive(Component)]
pub struct Hastened {
    pub timer: Timer,
}

impl Hastened {
    pub fn new(duration: Duration) -> Self {
        Self {
            timer: Timer::new(duration, TimerMode::Once),
        }
    }
}

pub fn on_hastened(
    trigger: Trigger<HasteEvent>,
    time: Res<Time>,
    battle: Res<Battle>,
    q_source: Query<&Name>,
    q_target: Query<&Name, With<Item>>,
    q_haste: Query<(&Haste, &Name)>,
    mut q_hastened_items: Query<&mut Hastened>,
    mut commands: Commands,
) {
    let HasteEvent { source, target, with } = trigger.event();
    
    let source_name = q_source.get(*source).expect("source should exist");
    let target_name = q_target.get(*target).expect("target should exist");
    
    let (haste, item_name) = q_haste.get(*with).expect("haste source should exist");
    
    // If the target already has the Hastened component, extend its duration
    match q_hastened_items.get_mut(*target) {
        Ok(mut hastened) => {
            let remaining = hastened.timer.remaining();
            hastened.timer.set_duration(remaining + haste.duration);
            hastened.timer.reset();
            
            eprintln!(
                "{:?}: {:?} applied additional haste to {} with {}! Total duration: {:?}",
                battle.elapsed(time.elapsed_secs_f64()),
                source_name,
                target_name,
                item_name,
                hastened.timer.duration(),
            );
        }
        Err(_) => {
            // Add a new Hastened component to the target item
            commands.entity(*target).insert(Hastened::new(haste.duration));
            
            eprintln!(
                "{:?}: {:?} hastened {} with {} for {:?}!",
                battle.elapsed(time.elapsed_secs_f64()),
                source_name,
                target_name,
                item_name,
                haste.duration,
            );
        }
    }
}