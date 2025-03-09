use bevy::prelude::*;
use models::speed::Speed;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        let _ = app.add_systems(Update, move_enemy);
    }
}

fn move_enemy(
    time: Res<Time>,
    enemy: Query<(Entity, &Speed), With<Transform>>,
    mut transforms: Query<&mut Transform>,
) {
    for (entity, speed) in &enemy {
        if let Ok(mut transform) = transforms.get_mut(entity) {
            info!(
                "Moving enemy {} by {} from {}",
                entity, speed.0, transform.translation
            );
            transform.translation.x += speed.0 * time.delta_secs();
        }
    }
}
