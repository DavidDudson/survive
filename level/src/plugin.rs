use crate::ground::Ground;
use bevy::prelude::*;
use castle::castle::Castle;
use enemy::enemy::Enemy;
use enemy::peasant::Peasant;
use models::game_states::GameState;
use bevy_rapier2d::prelude::*;
use models::grounded::Grounded;
use models::scenery::Scenery;

// New resource for spawn timer
#[derive(Resource)]
struct SpawnTimer(Timer);

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(
                OnExit(GameState::Playing),
                cleanup.run_if(not(in_state(GameState::Paused))),
            )
            .add_systems(Update, spawn_waves.run_if(in_state(GameState::Playing)))
            .add_systems(
                Update,
                check_grounded.run_if(in_state(GameState::Playing)),
            )
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

fn check_grounded(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &Velocity), With<Scenery>>,
    ground_query: Query<&Transform, With<Ground>>,
) {
    if let Ok(ground_transform) = ground_query.get_single() {
        for (entity, transform, velocity) in &mut query {
            let is_grounded = transform.translation.y - 16.0 <= ground_transform.translation.y + 500.0
                && velocity.linvel.y.abs() < 0.1;
            
            commands.entity(entity).insert(Grounded(is_grounded));
        }
    }
}
