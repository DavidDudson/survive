use bevy::color::palettes::basic::GREEN;
use bevy::color::palettes::css::GREY;
use bevy::prelude::*;

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
        Transform::from_translation(Vec3::new(0., -270., 0.)),
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(300., 150.))),
        MeshMaterial2d(materials.add(Color::from(GREY))),
        Transform::from_translation(Vec3::new(1920. / 4., 75., 1.)),
    ));
    commands.spawn((
        Sprite::default(),
        Transform::from_scale(Vec3::new(64., 64., 0.)).with_translation(Vec3::new(0., 32., 0.)),
    ));
    info!("Spawning complete");
}
