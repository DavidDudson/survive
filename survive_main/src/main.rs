use bevy::prelude::*;
use castle::plugin::CastlePlugin;
use ui::plugin::UiPlugin;

fn hello_world() {
    println!("Main Running");
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, UiPlugin, CastlePlugin))
        .add_systems(Startup, hello_world)
        .run();
}
