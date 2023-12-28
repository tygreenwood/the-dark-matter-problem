use bevy::prelude::*;

use crate::setup::configs::AppStates;

use self::systems::*;

mod components;
mod configs;
mod systems;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppStates::Game), setup_background)
            .add_systems(
                Update,
                (move_background, animate_background).run_if(in_state(AppStates::Game)),
            )
            .add_systems(OnExit(AppStates::Game), cleanup);
    }
}
