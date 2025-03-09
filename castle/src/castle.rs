use bevy::prelude::*;
use bevy_rapier2d::prelude::RigidBody;
use models::entity_health::EntityHealth;
use models::health::Health;
use models::name::Name;

#[derive(Component)]
#[require(Name(castle_name), EntityHealth(castle_health), RigidBody(castle_body))]
pub struct Castle;

fn castle_name() -> Name {
    Name("Castle".to_string())
}

fn castle_health() -> EntityHealth {
    EntityHealth(Health(100))
}

fn castle_body() -> RigidBody {
    RigidBody::Fixed
}
