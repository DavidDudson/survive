use crate::enemy::{Enemy, ENEMY_COLLISION_GROUP};
use bevy::color::palettes::css::WHITE;
use bevy::prelude::*;
use bevy_rapier2d::prelude::{ActiveEvents, Collider, Damping, LockedAxes};
use models::attack::Attack;
use models::draggable::Draggable;
use models::hardness::Hardness;
use models::health::Health;
use models::name::Name;
use models::speed::Speed;
use models::textured::Textured;

#[derive(Component)]
#[require(
    Enemy,
    Name(peasant_name),
    Textured(peasant_texture),
    Health(peasant_health),
    Speed(peasant_speed),
    Attack(peasant_attack)
)]
pub struct Peasant;

impl Peasant {
    pub fn spawn(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        info!("Spawning peasant");
        commands.spawn((
            Draggable,
            Peasant,
            Mesh2d(meshes.add(Rectangle::new(64., 64.))),
            MeshMaterial2d(materials.add(Color::from(WHITE))),
            Transform::from_xyz(-1920. / 2., 0., 0.),
            Collider::cuboid(64. / 2., 64. / 2.),
            LockedAxes::ROTATION_LOCKED,
            ENEMY_COLLISION_GROUP,
            ActiveEvents::CONTACT_FORCE_EVENTS,
            Damping {
                linear_damping: 0.5,
                angular_damping: 1.0,
            },
            Hardness(1)
        ));
    }
}

fn peasant_name() -> Name {
    Name("Peasant".to_string())
}

fn peasant_health() -> Health {
    Health(5)
}

fn peasant_speed() -> Speed {
    Speed(100.)
}

fn peasant_attack() -> Attack {
    Attack::melee(Health(1))
}

fn peasant_texture() -> Textured {
    Textured {
        file: "enemy/Enemy.png".to_string(),
    }
}
