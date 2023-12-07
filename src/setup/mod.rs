use bevy::{prelude::*, window::close_on_esc};

use self::resources::{DisplayResolution, DisplayScale};

pub mod resources;
pub mod systems;

pub const WINDOW_WIDTH: f32 = 1024.0;
pub const WINDOW_HEIGHT: f32 = 720.0;

pub const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;
pub const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / -2.0;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DisplayResolution>()
            .init_resource::<DisplayScale>()
            .add_systems(Update, close_on_esc);
    }
}
