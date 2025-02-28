use crate::effects::freeze::Frozen;
use crate::effects::haste::Hastened;
use crate::effects::slow::Slowed;
use crate::fighting::TickEvent;
use bevy::prelude::*;
use std::time::Duration;

#[allow(clippy::type_complexity)]
pub fn tick_usable(
    _trigger: Trigger<TickEvent>,
    _time: Res<Time>,
    mut query: Query<(
        &mut Usable,
        Entity,
        Option<&mut Hastened>,
        Option<&mut Slowed>,
        Option<&mut Frozen>,
    )>,
    mut commands: Commands,
) {
    // Track entities that need condition removal
    let mut entities_to_unfreeze = Vec::new();
    let mut entities_to_unhaste = Vec::new();
    let mut entities_to_unslow = Vec::new();
    
    // Process all items and their conditions in a single loop
    for (mut usable, entity, mut maybe_hastened, mut maybe_slowed, mut maybe_frozen) in query.iter_mut() {
        // First, tick condition timers if they exist
        if let Some(ref mut frozen) = maybe_frozen {
            frozen.timer.tick(Duration::from_millis(100));
            if frozen.timer.just_finished() {
                eprintln!("Frozen condition expired on {:?}", entity);
                entities_to_unfreeze.push(entity);
            }
        }
        
        if let Some(ref mut hastened) = maybe_hastened {
            hastened.timer.tick(Duration::from_millis(100));
            if hastened.timer.just_finished() {
                eprintln!("Hastened condition expired on {:?}", entity);
                entities_to_unhaste.push(entity);
            }
        }
        
        if let Some(ref mut slowed) = maybe_slowed {
            slowed.timer.tick(Duration::from_millis(100));
            if slowed.timer.just_finished() {
                eprintln!("Slowed condition expired on {:?}", entity);
                entities_to_unslow.push(entity);
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
            commands.trigger_targets(UseEvent {}, entity);
            // reset the cooldown
            usable.cooldown = Timer::new(timer.duration(), TimerMode::Once);
        }
    }
    
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
