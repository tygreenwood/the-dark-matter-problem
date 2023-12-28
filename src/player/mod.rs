use bevy::prelude::*;

use crate::setup::configs::AppStates;

use self::systems::*;

pub mod components;
mod configs;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppStates::Game), setup_player)
            .add_systems(
                Update,
                (movement, jump, fall, rise, animate_sprite).run_if(in_state(AppStates::Game)),
            )
            .add_systems(OnExit(AppStates::Game), cleanup);
    }
}
