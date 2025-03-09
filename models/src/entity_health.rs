use crate::health::Health;
use bevy::prelude::Component;
use derive_more::{Add, AddAssign, Display, From, Mul, MulAssign};

#[derive(Component, From, Add, Mul, MulAssign, AddAssign, Display, Debug)]
pub struct EntityHealth(pub Health);

impl Default for EntityHealth {
    fn default() -> Self {
        EntityHealth(Health(10))
    }
}
