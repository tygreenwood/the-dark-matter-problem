use bevy::prelude::{App, Plugin, Startup, Update};

use systems::{move_background, setup_background};

use self::systems::animate_background;

mod components;
mod configs;
mod systems;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_background)
            .add_systems(Update, (move_background, animate_background));
    }
}
