use bevy::prelude::*;
use castle::plugin::CastlePlugin;
use enemy::plugin::EnemyPlugin;
use level::plugin::LevelPlugin;
use ui::plugin::UiPlugin;

fn hello_world() {
    println!("Main Running");
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            UiPlugin,
            CastlePlugin,
            LevelPlugin,
            EnemyPlugin,
        ))
        .add_systems(Startup, hello_world)
        .run();
}
