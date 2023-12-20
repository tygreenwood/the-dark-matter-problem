use bevy::{prelude::*, window::close_on_esc};

use resources::DisplayScale;
use systems::setup_graphics;

pub mod configs;
pub mod resources;
mod systems;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DisplayScale>()
            .add_systems(PreStartup, setup_graphics)
            .add_systems(Update, close_on_esc);
    }
}
