use bevy::prelude::*;

use crate::setup::configs::AppStates;

use self::systems::*;

pub mod components;
mod configs;
mod systems;

pub struct WheelPlugin;

impl Plugin for WheelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppStates::Game), setup_wheel)
            .add_systems(Update, (spin, spin_wheel).run_if(in_state(AppStates::Game)))
            .add_systems(OnExit(AppStates::Game), cleanup);
    }
}
