// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

mod characters;
mod effects;
mod fighting;
mod items;
mod loading;

use std::time::Duration;
use bevy::prelude::*;
use bevy::{app::ScheduleRunnerPlugin};

fn main() {
    App::new()
        .add_plugins((
            // Run ScheduleRunnerPlugin as fas as possible (no waiting):
            DefaultPlugins.set(ScheduleRunnerPlugin::default()),
            loading::LoadingPlugin {},
            fighting::FightingPlugin {},
            effects::EffectsPlugin {},
            items::ItemsPlugin {},
        ))
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(100)))
        .init_state::<GameState>()
        .add_systems(OnEnter(GameState::Results), exit_game)
        .run();
}

fn exit_game(mut app_exit_events: EventWriter<AppExit>) {
    eprintln!("Game over! Exiting...");
    app_exit_events.write(AppExit::Success);
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    Loading,
    Fight,
    Results,
}
