use bevy::prelude::{App, Plugin, Startup};

use self::systems::{animate_sprite, setup_mushrooms};

mod components;
mod systems;

pub struct MushroomsPlugin;

impl Plugin for MushroomsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_mushrooms, animate_sprite));
    }
}
