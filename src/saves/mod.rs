use bevy::prelude::{App, Plugin, PreStartup, Update};

use self::{
    components::{PositionSaveComponent, PositionSaveInformation, SaveEvent},
    systems::{check_for_save, load_scene_system, move_player, save_scene_system},
};

pub mod components;
mod configs;
mod systems;

pub struct SavesPlugin;

impl Plugin for SavesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PositionSaveInformation>()
            .add_event::<SaveEvent>()
            .register_type::<PositionSaveComponent>()
            .add_systems(PreStartup, load_scene_system)
            .add_systems(Update, (check_for_save, save_scene_system, move_player));
    }
}
