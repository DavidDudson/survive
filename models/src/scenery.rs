use crate::hardness::Hardness;
use bevy::prelude::Component;
use bevy_rapier2d::dynamics::RigidBody;
use bevy_rapier2d::prelude::Collider;

#[derive(Component, Default)]
#[require(RigidBody(scenery_body), Collider, Hardness)]
pub struct Scenery;

fn scenery_body() -> RigidBody {
    RigidBody::Fixed
}
