use crate::scenery::Scenery;
use bevy::prelude::*;
use models::name::Name;

#[derive(Component, Default)]
#[require(Name(ground_name), Scenery)]
pub struct Ground;

fn ground_name() -> Name {
    Name("Ground".to_string())
}
