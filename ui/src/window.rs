use bevy::prelude::*;
use bevy::window::*;

const SCREEN_WIDTH: f32 = 1920.;
const SCREEN_HEIGHT: f32 = 1080.;

pub fn window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: String::from("Survive!"),
            name: Some(String::from("survive.app")),
            resolution: WindowResolution::from((SCREEN_WIDTH, SCREEN_HEIGHT)),
            present_mode: PresentMode::AutoNoVsync,
            fit_canvas_to_parent: true,
            prevent_default_event_handling: false,
            window_theme: Some(WindowTheme::Dark),
            ..default()
        }),
        ..default()
    }
}
