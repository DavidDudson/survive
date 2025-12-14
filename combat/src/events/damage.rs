use bevy::prelude::{Entity, Message};
use derive_more::Display;
use models::attack::Attack;

#[derive(Message, Clone, Copy, Debug, Display)]
#[display("{source}, attacked {target} for {attack}")]
pub struct DamageEvent {
    pub target: Entity,
    pub attack: Attack,
    pub source: Entity,
}

#[derive(Message, Clone, Copy, Debug, Display)]
#[display("{target} was killed by {source} with a final by {attack}")]
pub struct DeathEvent {
    pub target: Entity,
    pub attack: Attack,
    pub source: Entity,
}
