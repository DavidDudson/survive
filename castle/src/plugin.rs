use crate::castle::Castle;
use bevy::prelude::*;
use models::name::Name;

pub struct CastlePlugin;

fn spawn_castle(mut commands: Commands) {
    commands.spawn((Castle, Name("Castle".to_string())));
}

impl Plugin for CastlePlugin {
    fn build(&self, app: &mut App) {
        println!("Castle Plugin Built");
        app.add_systems(Startup, spawn_castle);
    }
}
