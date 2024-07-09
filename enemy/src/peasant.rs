use crate::enemy::Enemy;
use bevy::prelude::*;

#[derive(Component)]
pub struct Peasant;

impl Enemy for Peasant {
    fn texture(&self) -> String {
        "enemy/Enemy.png".to_string()
    }
}
