use crate::enemy::Enemy;
use bevy::prelude::*;
use models::attack::Attack;
use models::entity_health::EntityHealth;
use models::health::Health;
use models::name::Name;
use models::speed::Speed;
use models::textured::Textured;

#[derive(Component)]
#[require(
    Enemy,
    Name(peasant_name),
    Textured(peasant_texture),
    EntityHealth(peasant_health),
    Speed(peasant_speed),
    Attack(peasant_attack)
)]
pub struct Peasant;

fn peasant_name() -> Name {
    Name("Peasant".to_string())
}

fn peasant_health() -> EntityHealth {
    EntityHealth(Health(100))
}

fn peasant_speed() -> Speed {
    Speed(10.)
}

fn peasant_attack() -> Attack {
    Attack::physical(Health(1))
}

fn peasant_texture() -> Textured {
    Textured {
        file: "enemy/Enemy.png".to_string(),
    }
}
