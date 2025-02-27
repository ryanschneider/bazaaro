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
            effects::EffectsPlugin {},
            items::ItemsPlugin {},
        ))
        .insert_resource(Time::<Virtual>::from_max_delta(Duration::from_millis(100)))
        .init_state::<GameState>()
        .add_systems(OnEnter(GameState::Results), exit_game)
        .run();
}

fn exit_game(mut app_exit_events: EventWriter<AppExit>) {
    eprintln!("Game over! Exiting...");
    app_exit_events.send(AppExit::Success);
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    Loading,
    Fight,
    Results,
}
