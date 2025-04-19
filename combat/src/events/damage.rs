use bevy::prelude::{Entity, Event};
use derive_more::Display;
use models::attack::Attack;

#[derive(Event, Clone, Copy, Debug, Display)]
#[display("{source}, attacked {target} for {attack}")]
pub struct DamageEvent {
    pub target: Entity,
    pub attack: Attack,
    pub source: Entity,
}
