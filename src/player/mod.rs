use bevy::prelude::*;

use crate::setup::configs::AppStates;

use self::systems::*;

pub mod components;
pub mod configs;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppStates::Game), setup_player)
            .add_systems(
                Update,
                (
                    (
                        movement,
                        jump,
                        vertical_velocity,
                        animate_sprite,
                        remove_jump,
                        dash,
                        dash_cooldown,
                    )
                        .run_if(in_state(AppStates::Game)),
                    gamepad_connections,
                ),
            )
            .add_systems(OnExit(AppStates::Game), cleanup);
    }
}
