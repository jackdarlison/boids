use bevy::prelude::*;

use crate::simulation_schedule::InSimulationSchedule;

const CAMERA_DISTANCE: f32 = 300.0;
const CAMERA_PANNING_SPEED: f32 = 10.0;
const CAMERA_ZOOM_SPEED: f32 = 10.0;

#[derive(Component, Debug)]
pub struct Camera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, camera_control_2d.in_set(InSimulationSchedule::UserInput));
    }
}


fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, CAMERA_DISTANCE, 0.0)
                .looking_at(Vec3::ZERO, Vec3::Z),
            ..default()
        },
        Camera,
    ));
}

fn camera_control_2d(key_input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Camera>>) {
    if let Ok(mut transform) = query.get_single_mut() {
        if key_input.pressed(KeyCode::D) {
            transform.translation.x -= 1.0 * CAMERA_PANNING_SPEED;
        }
        if key_input.pressed(KeyCode::A) {
            transform.translation.x += 1.0 * CAMERA_PANNING_SPEED;
        }
        if key_input.pressed(KeyCode::W) {
            transform.translation.z += 1.0 * CAMERA_PANNING_SPEED;
        }
        if key_input.pressed(KeyCode::S) {
            transform.translation.z -= 1.0 * CAMERA_PANNING_SPEED;
        }
        if key_input.pressed(KeyCode::Q) {
            transform.translation.y += 1.0 * CAMERA_ZOOM_SPEED;
        }
        if key_input.pressed(KeyCode::E) {
            transform.translation.y -= 1.0 * CAMERA_ZOOM_SPEED;
        }
    }
}