use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use castle::castle::Castle;
use models::game_states::GameState;

pub struct CameraPlugin;

#[derive(Component)]
struct CameraDrag {
    dragging: bool,
    target_scale: f32,
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                castle_follow.run_if(not(in_state(GameState::Playing))),
                camera_drag.run_if(in_state(GameState::Playing)),
            ),
        );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        CameraDrag {
            dragging: false,
            target_scale: 1.0,
        },
    ));
}

fn castle_follow(
    target_query: Query<&Transform, With<Castle>>,
    mut camera_query: Query<
        (&mut Transform, &mut OrthographicProjection),
        (With<Camera>, Without<Castle>),
    >,
    time: Res<Time>,
) {
    // Get the target position
    if let Ok(target_transform) = target_query.get_single() {
        // Get the camera
        if let Ok((mut camera_transform, mut projection)) = camera_query.get_single_mut() {
            let target_position = Vec3::new(
                target_transform.translation.x,
                target_transform.translation.y,
                camera_transform.translation.z,
            );

            camera_transform.translation = camera_transform
                .translation
                .lerp(target_position, 5.0 * time.delta_secs());

            let target_scale = 0.5;
            let current_scale = projection.scale;

            projection.scale =
                current_scale + (target_scale - current_scale) * 2.0 * time.delta_secs();
        }
    }
}

// New system for camera dragging
fn camera_drag(
    mut camera_query: Query<(&mut Transform, &mut CameraDrag), With<Camera>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut scroll_events: EventReader<MouseWheel>,
    mut projection_query: Query<&mut OrthographicProjection, With<Camera>>,
    time: Res<Time>,
) {
    // Handle camera dragging
    let (mut camera_transform, mut camera_drag) = camera_query.single_mut();

    // Start dragging when middle mouse button is pressed
    if mouse_button.just_pressed(MouseButton::Left) {
        camera_drag.dragging = true;
    }

    // Stop dragging when middle mouse button is released
    if mouse_button.just_released(MouseButton::Left) {
        camera_drag.dragging = false;
    }

    // Move camera if dragging
    if camera_drag.dragging {
        for event in mouse_motion_events.read() {
            // Get the current projection scale to adjust drag sensitivity
            let projection = projection_query.single();
            let scale_factor = projection.scale;

            // Move camera in the opposite direction of mouse movement
            // Scale the movement by the projection scale to maintain consistent drag feel at different zoom levels
            camera_transform.translation.x -= event.delta.x * scale_factor;
            camera_transform.translation.y += event.delta.y * scale_factor;
        }
    }

    // Handle zooming with mouse wheel
    let mut zoom_delta = 0.0;
    for event in scroll_events.read() {
        zoom_delta -= event.y * 0.002; // Adjust sensitivity as needed
    }

    if zoom_delta != 0.0 {
        // Update the target scale based on the zoom delta
        camera_drag.target_scale = (camera_drag.target_scale + zoom_delta).clamp(0.1, 4.0);
    }

    // Smoothly interpolate towards the target scale
    if let Ok(mut projection) = projection_query.get_single_mut() {
        let zoom_speed = 5.0; // Adjust this value to control zoom smoothness (higher = faster)
        projection.scale = projection.scale
            + (camera_drag.target_scale - projection.scale) * zoom_speed * time.delta_secs();
    }
}
