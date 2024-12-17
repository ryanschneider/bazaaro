mod characters;
mod effects;
mod fighting;
mod items;
mod loading;

use bevy::prelude::*;
use bevy::{app::ScheduleRunnerPlugin, utils::Duration};

fn main() {
    App::new()
        .add_plugins((
            // Run ScheduleRunnerPlugin as fas as possible (no waiting):
            DefaultPlugins.set(ScheduleRunnerPlugin::default()),
            loading::LoadingPlugin {},
            fighting::FightingPlugin {},
            items::ItemsPlugin {},
        ))
        .insert_resource(Time::<Virtual>::from_max_delta(Duration::from_millis(100)))
        .init_state::<GameState>()
        .run();
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    Loading,
    Fight,
    Results,
}
