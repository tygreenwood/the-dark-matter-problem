use bevy::prelude::*;

use crate::{player::components::Player, setup::resources::DisplayScale};

use super::components::Camera;

pub fn setup_camera(mut commands: Commands, scale: Res<DisplayScale>) {
    commands.spawn((
        Camera2dBundle {
            projection: OrthographicProjection {
                scale: scale.0,
                near: -1000.,
                far: 1000.,
                ..default()
            },
            ..default()
        },
        Camera,
    ));
}

pub fn camera_follow_player(
    mut camera: Query<&mut Transform, With<Camera>>,
    player: Query<&GlobalTransform, With<Player>>,
) {
    if let Ok(player_transform) = player.get_single() {
        let mut camera_transform = camera.single_mut();
        camera_transform.translation.x = player_transform.translation().x;
    }
}
