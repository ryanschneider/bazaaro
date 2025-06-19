use std::ops::Deref;
use std::slice;
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

#[derive(Component, Debug)]
#[relationship(relationship_target = Items)]
pub struct ItemOf(Entity);

impl ItemOf {
    pub fn owner(&self) -> Entity {
        self.0
    }
}


#[derive(Default, Debug, Component)]
#[relationship_target(relationship = ItemOf)]
pub struct Items(Vec<Entity>);

impl<'a> IntoIterator for &'a Items {
    type Item = <Self::IntoIter as Iterator>::Item;

    type IntoIter = slice::Iter<'a, Entity>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Deref for Items {
    type Target = [Entity];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

