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
pub struct Seconds(pub u16);
