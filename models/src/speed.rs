use bevy::prelude::Component;
use derive_more::{Add, AddAssign, Display, From, Mul, MulAssign};

// Speed in pixels per second
#[derive(Component, From, Add, Mul, MulAssign, AddAssign, Display, Debug, Default)]
pub struct Speed(pub f32);
