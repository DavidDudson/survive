use bevy::prelude::*;

pub struct CastlePlugin;

impl Plugin for CastlePlugin {
    fn build(&self, _app: &mut App) {
        println!("Castle Plugin Built");
    }
}
