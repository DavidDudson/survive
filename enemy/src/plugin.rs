use bevy::prelude::*;
use bevy_rapier2d::plugin::ReadRapierContext;
use bevy_rapier2d::prelude::Velocity;
use models::draggable::Dragged;
use models::game_states::GameState;
use models::scenery::Scenery;
use models::speed::Speed;
use models::grounded::Grounded;
use crate::enemy::Enemy;
use crate::peasant::{Peasant, Limb, WalkingAnimation, check_grounded, animate_limbs};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                animate_limbs,
                update_limb_positions,
                animate_walking,
                move_enemy.run_if(in_state(GameState::Playing)),
            ),
        );
    }
}

fn update_limb_positions(
    mut query: Query<(&mut Transform, &Limb)>,
    parent_query: Query<&Transform, Without<Limb>>,
) {
    for (mut transform, limb) in query.iter_mut() {
        if let Ok(parent_transform) = parent_query.get(limb.parent) {
            transform.translation = parent_transform.translation + limb.offset.extend(0.);
        }
    }
}

fn animate_walking(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &WalkingAnimation, &Limb)>,
) {
    for (mut transform, animation, limb) in query.iter_mut() {
        // Use a combination of sine waves for more natural motion
        let base_angle = (time.elapsed().as_secs_f32() * animation.speed + animation.phase).sin();
        let secondary_angle = (time.elapsed().as_secs_f32() * animation.speed * 2.0 + animation.phase).sin() * 0.3;
        let angle = (base_angle + secondary_angle) * 0.4;
        transform.rotation = Quat::from_rotation_z(angle);
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
            let Ok(context) = rapier_context.single() else {
                continue;
            };
            if let Some(_) = context.contact_pair(entity, scenery.iter().next().unwrap()) {
                vel.linvel.x = speed.0
            }
        }
    }
}
