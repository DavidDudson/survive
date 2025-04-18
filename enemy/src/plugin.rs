use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use models::speed::Speed;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        let _ = app.add_systems(Update, move_enemy);
    }
}

fn move_enemy(
    _time: Res<Time>,
    enemy: Query<(Entity, &Speed), With<Velocity>>,
    mut external_forces: Query<&mut Velocity>,
) {
    for (entity, speed) in &enemy {
        if let Ok(mut external_force) = external_forces.get_mut(entity) {
            external_force.linvel.x = speed.0
        }
    }
}
