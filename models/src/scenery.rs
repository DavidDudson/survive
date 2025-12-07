use crate::hardness::Hardness;
use bevy::prelude::Component;
use bevy_rapier2d::dynamics::RigidBody;
use bevy_rapier2d::prelude::Collider;

#[derive(Component, Default)]
#[require(Rigidbody::Fixed, Collider, Hardness)]
pub struct Scenery;
