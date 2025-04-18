use crate::scenery::Scenery;
use bevy::color::palettes::css::GREEN;
use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;
use models::name::Name;

#[derive(Component, Default)]
#[require(Name(ground_name), Scenery)]
pub struct Ground;

impl Ground {
    pub fn spawn(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        commands.spawn((
            Ground,
            Mesh2d(meshes.add(Rectangle::new(1920., 540.))),
            MeshMaterial2d(materials.add(Color::from(GREEN))),
            Transform::from_xyz(0., -270., 0.),
            Collider::cuboid(1920. / 2., 520. / 2.),
        ));
    }
}

fn ground_name() -> Name {
    Name("Ground".to_string())
}
