use bevy::prelude::{App, Plugin, Startup, Update};

use self::{
    components::SaveEvent,
    systems::{check_for_save, load_scene_system, save_scene_system},
};

mod components;
mod systems;

pub struct SavesPlugin;

impl Plugin for SavesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SaveEvent>()
            .add_systems(Startup, load_scene_system)
            .add_systems(Update, (check_for_save, save_scene_system));
    }
}
