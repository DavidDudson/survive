use crate::damage_type::DamageType;
use crate::health::Health;
use bevy::prelude::Component;
use derive_more::Display;

#[derive(Component, Display, Debug)]
#[display("{damage}({damage_type})")]
pub struct Attack {
    damage: Health,
    damage_type: DamageType,
}

impl Attack {
    pub fn physical(damage: Health) -> Self {
        Self {
            damage,
            damage_type: DamageType::Physical,
        }
    }
}

impl Default for Attack {
    fn default() -> Self {
        Self::physical(Health(1))
    }
}
