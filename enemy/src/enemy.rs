use bevy::prelude::Component;
use bevy_rapier2d::geometry::{CollisionGroups, Group};
use bevy_rapier2d::prelude::{RigidBody, Velocity};
use models::attack::Attack;
use models::hardness::Hardness;
use models::health::Health;
use models::name::Name;
use models::speed::Speed;
use models::textured::Textured;

// Define a collision group for peasants
pub const ENEMY_GROUP: Group = Group::GROUP_1;
pub const ENEMY_COLLISION_GROUP: CollisionGroups =
    CollisionGroups::new(ENEMY_GROUP, Group::ALL.difference(ENEMY_GROUP));

#[derive(Component, Default)]
#[require(
    Name,
    Speed,
    Attack,
    Textured,
    Health,
    RigidBody(entity_body),
    Hardness,
    Velocity,
)]
pub struct Enemy {}

fn entity_body() -> RigidBody {
    RigidBody::Dynamic
}
