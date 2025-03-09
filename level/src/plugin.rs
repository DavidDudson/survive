use crate::ground::Ground;
use bevy::color::palettes::basic::GREEN;
use bevy::color::palettes::css::{GREY, WHITE};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use enemy::peasant::Peasant;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    spawn(commands, meshes, materials);
}

fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1920., 540.))),
        MeshMaterial2d(materials.add(Color::from(GREEN))),
        Transform::from_xyz(0., -270., 0.),
        Collider::cuboid(1920. / 2., 520. / 2.),
    ));
    commands.spawn((
        Ground,
        Mesh2d(meshes.add(Rectangle::new(300., 150.))),
        MeshMaterial2d(materials.add(Color::from(GREY))),
        Transform::from_xyz(1920. / 4., 75., 0.),
        Collider::cuboid(300. / 2., 150. / 2.),
    ));
    commands.spawn((
        Peasant,
        Mesh2d(meshes.add(Rectangle::new(64., 64.))),
        MeshMaterial2d(materials.add(Color::from(WHITE))),
        Transform::from_xyz(0., 60., 0.),
        Collider::cuboid(64. / 2., 64. / 2.),
    ));
    info!("Spawning complete");
}
