use bevy::prelude::*;

#[derive(Default, Component)]
#[require(Health, Items, Name)]
pub struct Character;

#[derive(Default, Component)]
#[require(Character)]
pub struct Hero;

#[derive(Default, Component)]
#[require(Character)]
pub struct Villain;

#[derive(Default, Component)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}

impl Health {
    pub fn starting(val: u32) -> Self {
        Self {
            current: val,
            max: val,
        }
    }

    pub fn reset(&mut self) {
        self.current = self.max;
    }
}

#[derive(Default, Component)]
pub struct Items {
    pub slots: Vec<Option<Entity>>,
}

impl Items {
    pub fn attach_to(self, owner: Entity, commands: &mut Commands) {
        let binding = self
            .slots
            .iter()
            .filter_map(|slot| slot.as_ref().copied())
            .collect::<Vec<_>>();
        let children: &[Entity] = binding.as_slice();
        commands.entity(owner).insert(self);
        commands.entity(owner).add_children(children);
    }
}
