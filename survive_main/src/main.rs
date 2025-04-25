use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use camera::plugin::CameraPlugin;
use castle::plugin::CastlePlugin;
use combat::plugin::CombatPlugin;
use enemy::plugin::EnemyPlugin;
use level::plugin::LevelPlugin;
use models::game_states::GameState;
use ui::plugin::UiPlugin;
use ui::window::window_plugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(window_plugin()),
            CameraPlugin,
            LogDiagnosticsPlugin::default(),
            // FrameTimeDiagnosticsPlugin,
            UiPlugin,
            CastlePlugin,
            LevelPlugin,
            EnemyPlugin,
            CombatPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        ))
        .init_state::<GameState>()
        .run();
}
