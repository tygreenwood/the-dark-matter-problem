use bevy::{prelude::*, window::close_on_esc};

use resources::DisplayScale;
use systems::setup_graphics;

pub mod resources;
mod systems;

pub const WINDOW_WIDTH: f32 = 1920.0;
pub const WINDOW_HEIGHT: f32 = 1080.0;

pub const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;
pub const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / -2.0;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DisplayScale>()
            .add_systems(PreStartup, setup_graphics)
            .add_systems(Update, close_on_esc);
    }
}
