use bevy::prelude::*;

use crate::setup::configs::AppStates;

use self::{
    components::{PositionSaveComponent, WheelSaveComponent},
    resources::{PositionSaveInformation, SaveGame, WheelSaveInformation},
    systems::*,
};

mod components;
mod configs;
pub mod resources;
mod systems;

pub struct SavesPlugin;

impl Plugin for SavesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PositionSaveInformation>()
            .init_resource::<WheelSaveInformation>()
            .init_resource::<SaveGame>()
            .register_type::<WheelSaveComponent>()
            .register_type::<PositionSaveComponent>()
            .add_systems(Startup, load_scene_system)
            .add_systems(
                Update,
                (
                    (save_scene_system, check_for_save).run_if(in_state(AppStates::Game)),
                    load_save,
                ),
            );
    }
}
