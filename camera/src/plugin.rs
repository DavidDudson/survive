use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use castle::castle::Castle;
use models::draggable::{Draggable, Dragged};
use models::game_states::GameState;

pub struct CameraPlugin;

#[derive(Component)]
struct CameraDrag {
    dragging: bool,
    target_scale: f32,
    last_drag: Option<f32>,
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                castle_follow.run_if(not(in_state(GameState::Playing))),
                camera_drag.run_if(in_state(GameState::Playing)),
                drag_system.run_if(in_state(GameState::Playing)),
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
            last_drag: None,
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
    dragged_entity: Query<&Dragged>,
    time: Res<Time>,
) {
    // Handle camera dragging
    let (mut camera_transform, mut camera_drag) = camera_query.single_mut();

    if !dragged_entity.is_empty() {
        camera_drag.last_drag = Some(time.elapsed_secs());
        camera_drag.dragging = false;
        return;
    }

    if !camera_drag.last_drag.is_none()
        && time.elapsed_secs() - camera_drag.last_drag.unwrap() < 0.5
    {
        camera_drag.dragging = false;
        return;
    }

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

// New system for dragging peasants
fn drag_system(
    // Assuming you have a Peasant component defined elsewhere
    mut query: Query<(Entity, &mut Transform, Option<&Dragged>), With<Draggable>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
) {
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();

    // Get the current mouse position in world coordinates
    let cursor_position = if let Some(cursor_position) = window.cursor_position() {
        // Convert screen position to world position
        if let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
            world_position
        } else {
            return; // Cursor not in world
        }
    } else {
        return; // Cursor not in window
    };

    // Check if we're already dragging a peasant
    let mut dragging = false;
    for (entity, mut transform, dragged) in query.iter_mut() {
        if dragged.is_some() {
            dragging = true;

            // Update the position of the dragged peasant to follow the cursor
            transform.translation.x = cursor_position.x;
            transform.translation.y = cursor_position.y;

            // If mouse button is released, stop dragging
            if mouse_button.just_released(MouseButton::Left) {
                commands.entity(entity).remove::<Dragged>();
            }

            break; // Only one peasant can be dragged at a time
        }
    }

    // If we're not already dragging a peasant and the left mouse button was just pressed,
    // check if we clicked on a peasant
    if !dragging && mouse_button.just_pressed(MouseButton::Left) {
        for (entity, transform, _) in query.iter() {
            // Simple distance check to see if we clicked on this peasant
            // You might want to use a more sophisticated collision detection system
            let peasant_pos = Vec2::new(transform.translation.x, transform.translation.y);
            let distance = cursor_position.distance(peasant_pos);

            // If the cursor is close enough to the peasant, start dragging it
            // Adjust the threshold based on your peasant's size
            if distance < 40.0 {
                commands.entity(entity).insert(Dragged);
                break; // Only pick up one peasant at a time
            }
        }
    }
}
