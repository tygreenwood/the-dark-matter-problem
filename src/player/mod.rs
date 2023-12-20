use bevy::prelude::*;

use systems::*;

pub mod components;
mod configs;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, (movement, jump, fall, rise, animate_sprite));
    }
}
