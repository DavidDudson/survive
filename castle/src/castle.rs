use bevy::color::palettes::css::GREY;
use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody};
use models::health::Health;
use models::name::Name;

#[derive(Component)]
#[require(Name(castle_name), Health(castle_health), RigidBody(castle_body))]
pub struct Castle;

impl Castle {
    pub fn spawn(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        commands.spawn((
            Castle,
            Mesh2d(meshes.add(Rectangle::new(300., 150.))),
            MeshMaterial2d(materials.add(Color::from(GREY))),
            Transform::from_xyz(1920. / 4., 75., 0.),
            Collider::cuboid(300. / 2., 150. / 2.),
        ));
    }
}

fn castle_name() -> Name {
    Name("Castle".to_string())
}

fn castle_health() -> Health {
    Health(20)
}

fn castle_body() -> RigidBody {
    RigidBody::Fixed
}
