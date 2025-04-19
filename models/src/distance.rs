use bevy::prelude::Component;
use derive_more::{Add, AddAssign, Display, From, Mul, MulAssign, Sub, SubAssign};

#[derive(
    Component,
    From,
    Add,
    Mul,
    MulAssign,
    AddAssign,
    Sub,
    SubAssign,
    Display,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Debug,
    Clone,
    Copy,
    Default,
)]
pub struct Distance(pub u16);

impl From<f32> for Distance {
    fn from(value: f32) -> Self {
        Distance(value.round() as u16)
    }
}

impl From<f64> for Distance {
    fn from(value: f64) -> Self {
        Distance(value.round() as u16)
    }
}
