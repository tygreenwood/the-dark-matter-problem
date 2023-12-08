use bevy::prelude::{App, Component, Plugin, Startup, Update};

use systems::{camera_follow_player, setup_camera};

pub mod components;
pub mod systems;

#[derive(Component)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, camera_follow_player);
    }
}
