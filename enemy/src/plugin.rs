use bevy::prelude::*;
use bevy_rapier2d::plugin::ReadRapierContext;
use bevy_rapier2d::prelude::Velocity;
use models::draggable::Dragged;
use models::game_states::GameState;
use models::scenery::Scenery;
use models::speed::Speed;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        let _ = app.add_systems(Update, move_enemy.run_if(in_state(GameState::Playing)));
    }
}

fn move_enemy(
    _time: Res<Time>,
    enemy: Query<(Entity, &Speed), (With<Velocity>, Without<Dragged>)>,
    scenery: Query<Entity, With<Scenery>>,
    mut velocity: Query<&mut Velocity>,
    rapier_context: ReadRapierContext,
) {
    for (entity, speed) in &enemy {
        if let Ok(mut vel) = velocity.get_mut(entity) {
            if let Some(_) = rapier_context
                .single()
                .contact_pair(entity, scenery.iter().next().unwrap())
            {
                vel.linvel.x = speed.0
            }
        }
    }
}
