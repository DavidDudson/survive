use crate::events::damage::DamageEvent;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use castle::castle::Castle;
use models::attack::Attack;
use models::distance::Distance;
use models::game_states::GameState;
use models::health::Health;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        let _ = app.add_event::<DamageEvent>()
            .add_systems(FixedUpdate, display_collisions)
            .add_systems(
            Update,
            (detect_attacks, apply_damage).run_if(in_state(GameState::Playing)),
        );
    }
}

fn display_collisions(collision_events: Query<&ActiveEvents>) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
    }
}

// System to detect when characters are in range and send damage events
fn detect_attacks(
    time: Res<Time>,
    mut character_query: Query<(Entity, &Transform, &mut Attack), With<Attack>>,
    castle_query: Query<(Entity, &Transform), With<Castle>>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    if let Ok((castle_entity, castle_transform)) = castle_query.get_single() {
        for (character_entity, transform, mut attack) in &mut character_query {
            let center_distance = transform.translation.distance(castle_transform.translation);

            if Distance::from(center_distance) <= attack.range {
                if attack.last.is_none()
                    || time.elapsed_secs() - attack.last.unwrap() > attack.cooldown.0.into()
                {
                    attack.last = Some(time.elapsed_secs());
                    damage_events.send(DamageEvent {
                        target: castle_entity,
                        attack: attack.clone(),
                        source: character_entity,
                    });
                }
            }
        }
    }
}

// System to apply damage from events
fn apply_damage(
    mut damage_events: EventReader<DamageEvent>,
    mut health_query: Query<&mut Health>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in damage_events.read() {
        if let Ok(mut health) = health_query.get_mut(event.target) {
            if *health > event.attack.damage {
                *health -= event.attack.damage;
                info!("Castle Health {}", *health)
            } else {
                health.0 = 0;
                info!("Castle Died {}", *health);
                next_state.set(GameState::GameOver);
            }
        }
    }
}
