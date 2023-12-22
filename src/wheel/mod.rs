use bevy::prelude::{App, Plugin, Startup, Update};

use self::systems::{setup_wheel, spin, spin_wheel};

pub mod components;
mod configs;
mod systems;

pub struct WheelPlugin;

impl Plugin for WheelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_wheel)
            .add_systems(Update, (spin, spin_wheel));
    }
}
