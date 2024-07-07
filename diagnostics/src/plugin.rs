use bevy::prelude::*;
use bevy::diagnostic::LogDiagnosticsPlugin;

pub struct DiagnosticsPlugin;

#[cfg(debug_assertions)]
impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        println!("DiagnosticsPlugin Plugin Built");
        app.add_plugins(LogDiagnosticsPlugin::default());
    }
}