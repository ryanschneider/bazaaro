use crate::effects::freeze::FreezeEvent;
use bevy::prelude::*;
use crate::items::usable_with_targeted_effect::{EffectContext, UsableWithTargetedEffect};
pub fn freezer() -> UsableWithTargetedEffect {
    UsableWithTargetedEffect::new(freeze_effect)
}

fn freeze_effect(EffectContext{ owner, target, source}: EffectContext, commands: &mut Commands) {
    commands.trigger(FreezeEvent::new(owner, target, source));
}
