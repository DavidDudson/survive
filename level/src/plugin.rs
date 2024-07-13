use bevy::color::palettes::basic::GREEN;
use bevy::color::palettes::css::GREY;
use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;

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
    commands.spawn(Camera2dBundle::default());
    spawn(commands, meshes, materials);
}

fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(ColorMesh2dBundle {
        mesh: Mesh2dHandle::from(meshes.add(Rectangle::new(1920., 540.))),
        transform: Transform::from_translation(Vec3::new(0., -270., 0.)),
        material: materials.add(Color::from(GREEN)),

        ..default()
    });
    commands.spawn(ColorMesh2dBundle {
        mesh: Mesh2dHandle::from(meshes.add(Rectangle::new(300., 150.))),
        material: materials.add(Color::from(GREY)),
        transform: Transform::from_translation(Vec3::new(1920. / 4., 75., 1.)),
        ..default()
    });
    commands.spawn(SpriteBundle {
        transform: Transform::from_scale(Vec3::new(64., 64., 0.))
            .with_translation(Vec3::new(0., 32., 0.)),
        ..Default::default()
    });
    info!("Spawning complete");
}
