use bevy::prelude::*;

use crate::setup::configs::AppStates;

use self::systems::*;

pub mod components;
pub mod configs;
mod systems;

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppStates::Game), setup_entities)
            .add_systems(OnExit(AppStates::Game), cleanup);
    }
}
