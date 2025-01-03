#![allow(clippy::type_complexity)]

use bevy::core_pipeline::bloom::Bloom;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::render::camera::Viewport;

use crate::player_plugin::Player;
use crate::robot_factory::robot_parts::{Robot, RobotHead, RobotBody};

/// Camera lerp factor.
const CAM_LERP_FACTOR: f32 = 10.7;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, update_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            hdr: true, // 1. HDR is required for bloom
            ..default()
        },
        Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
        Bloom::default(),           // 3. Enable bloom for the camera
    ));
}

/// Update the camera position by tracking the player.
fn update_camera(
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), (With<Camera2d>)>,
    player: Query<&GlobalTransform, (With<RobotHead>, Without<Camera2d>)>,
    kb_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut camera_projection) = camera_query.single_mut();

    let Ok(player) = player.get_single() else {
        return;
    };

    if kb_input.pressed(KeyCode::NumpadAdd) {
        camera_projection.scale += 1. * time.delta_secs();
    }
    if kb_input.pressed(KeyCode::NumpadSubtract) {
        camera_projection.scale -= 1. * time.delta_secs();
    }

    let Vec3 { x, y, .. } = player.translation();
    let direction = Vec3::new(x, y, transform.translation.z);

    // Applies a smooth effect to camera movement using interpolation between
    // the camera position and the player position on the x and y axes.
    // Here we use the in-game time, to get the elapsed time (in seconds)
    // since the previous update. This avoids jittery movement when tracking
    // the player.
    transform.translation = transform
        .translation
        .lerp(direction, time.delta_secs() * CAM_LERP_FACTOR);
}
