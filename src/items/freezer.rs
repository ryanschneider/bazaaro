use crate::effects::freeze::FreezeEvent;
use crate::items::usable_with_targeted_effect::UsableWithTargetedEffect;
pub fn freezer() -> UsableWithTargetedEffect {
    UsableWithTargetedEffect::new(|source, target, with, commands| {
        commands.trigger(FreezeEvent::new(source, target, with))
    })
}
