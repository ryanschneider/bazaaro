// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

mod characters;
mod effects;
mod fighting;
mod items;
mod loading;

use bevy::app::ScheduleRunnerPlugin;
use bevy::log::tracing_subscriber;
use bevy::log::tracing_subscriber::{fmt, prelude::*};
use bevy::prelude::*;
use std::time::Duration;

fn main() {
    let format = fmt::format()
        .compact()
        .without_time() // Don't include timestamp
        .with_target(false) // Don't include module (target)
        .with_level(true);

    let layer = tracing_subscriber::fmt::layer()
        .event_format(format) // Apply your custom format
        .with_writer(std::io::stdout) // Or your desired writer
        .boxed();

    tracing_subscriber::registry().with(layer).init();

    App::new()
        .add_plugins((
            // Run ScheduleRunnerPlugin as fas as possible (no waiting):
            DefaultPlugins
                .set(ScheduleRunnerPlugin::default())
                .set(bevy::log::LogPlugin {
                    level: bevy::log::Level::INFO,
                    // filter: "wgpu=warn,bevy_ecs=info".to_string(),
                    ..default()
                }),
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
    info!("Game over! Exiting...");
    app_exit_events.write(AppExit::Success);
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    Loading,
    Fight,
    Results,
}
