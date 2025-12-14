use bevy::color::palettes::css::GREY;
use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody};
use models::hardness::Hardness;
use models::health::Health;
use models::name::Name;

#[derive(Component)]
#[require(Name, Health, RigidBody::Fixed)]
pub struct Castle;

impl Castle {
    pub fn spawn(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        commands.spawn((
            Castle,
            Name("Castle".to_string()),
            Health(20),
            Mesh2d(meshes.add(Rectangle::new(300., 150.))),
            MeshMaterial2d(materials.add(Color::from(GREY))),
            Transform::from_xyz(1920. / 4., 75., 0.),
            Collider::cuboid(300. / 2., 150. / 2.),
            Hardness(10),
        ));
    }
}
