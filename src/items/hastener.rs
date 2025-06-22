use crate::effects::haste::HasteEvent;
use bevy::prelude::*;
use crate::items::usable_with_targeted_effect::{EffectContext, UsableWithTargetedEffect};
pub fn hastener() -> UsableWithTargetedEffect {
    UsableWithTargetedEffect::new(haste_effect)
}

fn haste_effect(EffectContext{ owner, target, source}: EffectContext, commands: &mut Commands) {
    commands.trigger(HasteEvent::new(owner, target, source));
}
