use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use castle::plugin::CastlePlugin;
use enemy::plugin::EnemyPlugin;
use level::plugin::LevelPlugin;
use ui::plugin::UiPlugin;
use ui::window::window_plugin;

fn hello_world() {
    println!("Main Running");
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(window_plugin()),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
            UiPlugin,
            CastlePlugin,
            LevelPlugin,
            EnemyPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        ))
        .add_systems(Startup, hello_world)
        .run();
}
