use crate::ground::Ground;
use bevy::prelude::*;
use castle::castle::Castle;
use enemy::enemy::Enemy;
use enemy::peasant::Peasant;
use models::game_states::GameState;

// New resource for spawn timer
#[derive(Resource)]
struct SpawnTimer(Timer);

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(OnExit(GameState::Playing), cleanup)
            .add_systems(Update, spawn_waves.run_if(in_state(GameState::Playing)))
            .insert_resource(SpawnTimer(Timer::from_seconds(5.0, TimerMode::Repeating)));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    Ground::spawn(&mut commands, &mut meshes, &mut materials);
    Castle::spawn(&mut commands, &mut meshes, &mut materials);
}

fn spawn_waves(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
) {
    // Tick the timer
    spawn_timer.0.tick(time.delta());

    // Check if the timer has finished
    if spawn_timer.0.just_finished() {
        Peasant::spawn(&mut commands, &mut meshes, &mut materials);
    }
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<Enemy>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
