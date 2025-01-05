use bevy::prelude::*;
use castle::castle::Castle;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, move_enemy)
    }

    // fn move_enemy(
    //     _time: Res<Time>,
    //     castle: Query<(Entity, &Castle), With<Transform>>,
    //     mut transforms: Query<&mut Transform>
    // ) {
    //     for (entity, castle) in &castle {
    //         if let Ok(mut transform) = transforms.get_mut(entity) {
    //             transform.with_translation(Vec3.);
    //         }
    //     }
    // }
}
