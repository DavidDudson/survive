use bevy::prelude::Component;
use derive_more::{Display, From};

#[derive(Component, From, Display)]
pub struct Name(pub String);

impl Default for Name {
    fn default() -> Self {
        Name("Unnamed".to_string())
    }
}
