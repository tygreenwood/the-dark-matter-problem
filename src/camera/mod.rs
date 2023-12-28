use bevy::prelude::*;

use crate::setup::configs::AppStates;

use self::systems::*;

pub mod components;
mod systems;

#[derive(Component)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera).add_systems(
            Update,
            camera_follow_player.run_if(in_state(AppStates::Game)),
        );
    }
}
