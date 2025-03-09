use bevy::prelude::Component;
use derive_more::{Add, AddAssign, Display, From, Mul, MulAssign};

#[derive(Component, From, Add, Mul, MulAssign, AddAssign, Display, Debug, Default)]
pub struct Health(pub i16);
