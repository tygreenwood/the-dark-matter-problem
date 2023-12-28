use bevy::prelude::*;

use crate::setup::configs::AppStates;

use self::systems::*;

mod components;
mod configs;
mod systems;

pub struct MushroomsPlugin;

impl Plugin for MushroomsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppStates::Game), setup_mushrooms)
            .add_systems(
                Update,
                (animate_sprite, check_mushroom_hop).run_if(in_state(AppStates::Game)),
            )
            .add_systems(OnExit(AppStates::Game), cleanup);
    }
}
