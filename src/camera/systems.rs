use bevy::prelude::{Camera2dBundle, Commands, GlobalTransform, Query, Transform, With};

use crate::player::components::Player;

use super::components::Camera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), Camera));
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
