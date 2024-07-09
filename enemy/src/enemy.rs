use bevy::prelude::*;
use bevy::sprite::SpriteBundle;

pub trait Enemy {
    fn texture(&self) -> String;
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub sprite: SpriteBundle,
}

impl Default for EnemyBundle {
    fn default() -> Self {
        EnemyBundle {
            sprite: SpriteBundle {
                transform: Transform::from_scale(Vec3::new(64.0, 64.0, 1.0)),
                ..Default::default()
            },
        }
    }
}
