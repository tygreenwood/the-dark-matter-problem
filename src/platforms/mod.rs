use bevy::prelude::{App, Color, Plugin, Startup};

pub mod components;
mod configs;
mod systems;

use systems::setup_entities;

pub const COLOR_PLATFORM: Color = Color::rgb(0.13, 0.13, 0.23);

pub const FLOOR_THICKNESS: f32 = 10.0;

pub const COLOR_FLOOR: Color = Color::rgb(0.45, 0.55, 0.66);

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_entities);
    }
}
