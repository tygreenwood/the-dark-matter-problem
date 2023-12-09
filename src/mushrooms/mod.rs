use bevy::prelude::{App, Plugin, Startup, Update};

use self::systems::{animate_sprite, setup_mushrooms};

mod components;
mod systems;

pub struct MushroomsPlugin;

impl Plugin for MushroomsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_mushrooms).add_systems(Update, animate_sprite);
    }
}
