use bevy::prelude::*;
use enemy::enemy::EnemyBundle;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    spawn(commands);
}

fn spawn(mut commands: Commands) {
    commands.spawn(EnemyBundle::default());
    info!("Spawned Enemy");
}
