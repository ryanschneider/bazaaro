use crate::effects::freeze::Frozen;
use crate::effects::haste::Hastened;
use crate::effects::slow::Slowed;
use crate::fighting::TickEvent;
use bevy::prelude::*;
use std::time::Duration;

pub fn tick_usable(
    _trigger: Trigger<TickEvent>,
    _time: Res<Time>,
    mut q_usable: Query<(&mut Usable, Entity)>,
    mut q_hastened: Query<(Entity, &mut Hastened)>,
    mut q_slowed: Query<(Entity, &mut Slowed)>,
    mut q_frozen: Query<(Entity, &mut Frozen)>,
    mut commands: Commands,
) {
    // First, tick all condition timers
    let mut entities_to_unfreeze = Vec::new();
    let mut entities_to_unhaste = Vec::new();
    let mut entities_to_unslow = Vec::new();
    
    for (entity, mut frozen) in q_frozen.iter_mut() {
        frozen.timer.tick(Duration::from_millis(100));
        if frozen.timer.just_finished() {
            eprintln!("Frozen condition expired on {:?}", entity);
            entities_to_unfreeze.push(entity);
        }
    }
    
    for (entity, mut hastened) in q_hastened.iter_mut() {
        hastened.timer.tick(Duration::from_millis(100));
        if hastened.timer.just_finished() {
            eprintln!("Hastened condition expired on {:?}", entity);
            entities_to_unhaste.push(entity);
        }
    }
    
    for (entity, mut slowed) in q_slowed.iter_mut() {
        slowed.timer.tick(Duration::from_millis(100));
        if slowed.timer.just_finished() {
            eprintln!("Slowed condition expired on {:?}", entity);
            entities_to_unslow.push(entity);
        }
    }

    // Then process the usable items
    q_usable.iter_mut().for_each(|(mut usable, entity)| {
        let timer = &mut usable.cooldown;
        
        // Check for conditions
        let is_frozen = q_frozen.contains(entity);
        let is_hastened = q_hastened.contains(entity);
        let is_slowed = q_slowed.contains(entity);
        
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
            commands.trigger_targets(UseEvent {}, entity);
            // reset the cooldown
            usable.cooldown = Timer::new(timer.duration(), TimerMode::Once);
        }
    });
    
    // Remove expired conditions at the end
    for entity in entities_to_unfreeze {
        commands.entity(entity).remove::<Frozen>();
    }
    
    for entity in entities_to_unhaste {
        commands.entity(entity).remove::<Hastened>();
    }
    
    for entity in entities_to_unslow {
        commands.entity(entity).remove::<Slowed>();
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
