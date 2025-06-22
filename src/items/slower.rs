use crate::effects::slow::SlowEvent;
use bevy::prelude::*;
use crate::items::usable_with_targeted_effect::{EffectContext, UsableWithTargetedEffect};
pub fn slower() -> UsableWithTargetedEffect {
    UsableWithTargetedEffect::new(slow_effect)
}

fn slow_effect(EffectContext{ owner, target, source}: EffectContext, commands: &mut Commands) {
    commands.trigger(SlowEvent::new(owner, target, source));
}
