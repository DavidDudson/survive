use bevy::prelude::Component;
use bevy_rapier2d::dynamics::RigidBody;
use models::attack::Attack;
use models::entity_health::EntityHealth;
use models::name::Name;
use models::speed::Speed;
use models::textured::Textured;

#[derive(Component, Default)]
#[require(Name, Speed, Attack, Textured, EntityHealth, RigidBody(entity_body))]
pub struct Enemy {}

fn entity_body() -> RigidBody {
    RigidBody::Dynamic
}
