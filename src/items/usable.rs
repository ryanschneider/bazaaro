use crate::fighting::TickEvent;
use bevy::prelude::*;
use std::time::Duration;

pub fn tick_usable(
    _: Trigger<TickEvent>,
    mut q_usable: Query<(&mut Usable, Entity)>,
    mut commands: Commands,
) {
    q_usable.iter_mut().for_each(|(mut usable, entity)| {
        let timer = &mut usable.cooldown;
        // TODO: haste/slow/freeze here
        timer.tick(Duration::from_millis(100));
        if timer.just_finished() {
            commands.trigger_targets(UseEvent {}, entity);
            // reset the cooldown
            usable.cooldown = Timer::new(timer.duration(), TimerMode::Once);
        }
    });
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
