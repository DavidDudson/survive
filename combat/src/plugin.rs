use crate::events::damage::{DamageEvent, DeathEvent};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use castle::castle::Castle;
use models::attack::Attack;
use models::distance::Distance;
use models::game_states::GameState;
use models::hardness::Hardness;
use models::health::Health;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        let _ = app
            .add_event::<DamageEvent>()
            .add_event::<DeathEvent>()
            .add_systems(
                Update,
                (
                    detect_attacks,
                    apply_damage,
                    handle_collisions,
                    handle_deaths,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn handle_collisions(
    query_hardness: Query<&Hardness>,
    mut collision_events: EventReader<ContactForceEvent>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for collision_event in collision_events.read() {
        let entity1 = collision_event.collider1;
        let entity2 = collision_event.collider2;
        let force = collision_event.max_force_magnitude;
        let force_to_damage = (force / 100000000.0).round().clamp(0.0, u16::MAX as f32) as u16;

        if force_to_damage < 1 {
            continue;
        }

        if let (Ok(h1), Ok(h2)) = (query_hardness.get(entity1), query_hardness.get(entity2)) {
            if h1 > h2 {
                info!(
                    "Received collision event: {:?} {:?} {:?}",
                    h1, h2, force_to_damage
                );
                damage_events.send(DamageEvent {
                    target: entity2,
                    attack: Attack::melee(Health(force_to_damage)),
                    source: entity1,
                });
            } else {
                info!("Received collision event: {:?} {:?} {:?}", h2, h1, force);
                damage_events.send(DamageEvent {
                    target: entity1,
                    attack: Attack::fall(Health(force_to_damage)),
                    source: entity2,
                });
            }
        } else {
            info!(
                "Not ok {:?} {:?}",
                query_hardness.get(entity1),
                query_hardness.get(entity2)
            );
        }
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
    mut death_events: EventWriter<DeathEvent>,
    mut health_query: Query<&mut Health>,
) {
    for event in damage_events.read() {
        if let Ok(mut health) = health_query.get_mut(event.target) {
            if *health > event.attack.damage {
                *health -= event.attack.damage;
            } else {
                health.0 = 0;
                death_events.send(DeathEvent {
                    target: event.target,
                    attack: event.attack,
                    source: event.source,
                });
            }
        }
    }
}

fn handle_deaths(
    castle: Query<&Castle>,
    mut damage_events: EventReader<DeathEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    for event in damage_events.read() {
        if let Ok(_) = castle.get(event.target) {
            next_state.set(GameState::GameOver);
        } else {
            commands.entity(event.target).despawn_recursive();
        }
    }
}
