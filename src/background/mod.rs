use bevy::prelude::{App, Plugin, Startup};

use systems::setup_background;

mod components;
mod systems;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_background);
    }
}
