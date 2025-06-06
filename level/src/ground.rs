use bevy::color::palettes::css::GREEN;
use bevy::prelude::*;
use bevy_rapier2d::prelude::{ActiveEvents, Collider};
use models::hardness::Hardness;
use models::name::Name;
use models::scenery::Scenery;

#[derive(Component, Default)]
#[require(Name(ground_name), Scenery, Hardness(ground_hardness))]
pub struct Ground;

impl Ground {
    pub fn spawn(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        commands.spawn((
            Ground,
            Mesh2d(meshes.add(Rectangle::new(5000., 1000.))),
            MeshMaterial2d(materials.add(Color::from(GREEN))),
            Transform::from_xyz(0., -480., 0.),
            Collider::cuboid(5000. / 2., 980. / 2.),
            ActiveEvents::COLLISION_EVENTS,
        ));
    }
}

fn ground_hardness() -> Hardness {
    Hardness(3)
}

fn ground_name() -> Name {
    Name("Ground".to_string())
}
