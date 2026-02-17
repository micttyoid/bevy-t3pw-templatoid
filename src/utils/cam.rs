use bevy::{camera::*, prelude::*};

use crate::game::player::Player;

pub const FOLLOW_CAMERA_TRESHOLD: f32 = 100.0; // Determine based on the character speed
pub const FOLLOW_CAMERA_MAX_SPEED: f32 = 1000.0;
pub const FOLLOW_CAMERA_BASE_SPEED: f32 = 4.5;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera);
    app.add_systems(Update, update_camera);
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize,
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn update_camera(
    player_query: Single<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    time: Res<Time>,
) {
    let player_transform = player_query;
    if let Ok(mut camera_transform) = camera_query.single_mut() {
        let camera_pos = camera_transform.translation.truncate();
        let player_pos = player_transform.translation.truncate();
        let d = camera_pos.distance(player_pos);

        // smoothing
        let factor = (d / FOLLOW_CAMERA_TRESHOLD)
            .clamp(1.0, FOLLOW_CAMERA_MAX_SPEED / FOLLOW_CAMERA_BASE_SPEED);
        let effective_speed = FOLLOW_CAMERA_BASE_SPEED * factor;

        let pos: Vec2 = camera_pos.lerp(player_pos, effective_speed * time.delta_secs());
        camera_transform.translation.x = pos.x;
        camera_transform.translation.y = pos.y;
    }
}
