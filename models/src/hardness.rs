use bevy::prelude::Component;
use derive_more::{Add, AddAssign, AsRef, Display, From, Mul, MulAssign, Sub, SubAssign};

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
    AsRef,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Debug,
    Clone,
    Copy,
    Default,
)]
pub struct Hardness(pub u16);
