use bevy::prelude::*;

use self::{configs::AppStates, resources::DisplayScale, systems::*};

pub mod configs;
pub mod resources;
mod systems;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DisplayScale>()
            .add_systems(PreStartup, setup_graphics)
            .add_systems(OnEnter(AppStates::Game), hide_cursor)
            .add_systems(Update, esc_to_menu.run_if(in_state(AppStates::Game)))
            .add_systems(OnExit(AppStates::Game), show_cursor);
    }
}
