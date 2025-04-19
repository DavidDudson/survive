use bevy::prelude::*;
use castle::castle::Castle;
use models::game_states::GameState;
use models::health::Health;

pub struct UiPlugin;


#[derive(Component)]
struct HealthText;

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct HUD;

#[derive(Component)]
struct GameOverMenu;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
            app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
                .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu)
                .add_systems(Update, button_system.run_if(in_state(GameState::MainMenu)))
                .add_systems(OnEnter(GameState::Playing), setup_hud)
                .add_systems(OnExit(GameState::Playing), cleanup_hud)
                .add_systems(Update, update_health_text.run_if(in_state(GameState::Playing)))
                .add_systems(OnEnter(GameState::GameOver), setup_game_over_menu)
                .add_systems(OnExit(GameState::GameOver), cleanup_game_over_menu);
    }
}


fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn((
           MainMenu,
            Text::new("Main Menu".to_string()),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Node::default()
            }
        ));
}

fn setup_game_over_menu(mut commands: Commands) {
    commands
        .spawn((
           GameOverMenu,
            Text::new("Game Over".to_string()),
            Node {
                position_type: PositionType::Absolute,
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                ..Node::default()
            }
        ));
}

fn setup_hud(mut commands: Commands) {
    commands
        .spawn((
            Text::new("Health: --".to_string()),
            HealthText,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                right: Val::Px(5.0),
                ..Node::default()
            }
        ));
}

fn update_health_text(
    castle_query: Query<&Health, With<Castle>>,
    mut text_query: Query<&mut Text, With<HealthText>>,
) {
    // Get the castle's health
    if let Ok(health) = castle_query.get_single() {
        // Get the health text
        if let Ok(mut text) = text_query.get_single_mut() {
            // Update the text with the current health value
            text.0 = format!("Health: {}", health.0);
        }
    }
}

fn button_system(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Playing);
        }
    }
}


fn cleanup_hud(mut commands: Commands, query: Query<Entity, With<HUD>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn cleanup_game_over_menu(mut commands: Commands, query: Query<Entity, With<GameOverMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}