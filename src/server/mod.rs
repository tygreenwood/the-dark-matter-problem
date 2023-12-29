use bevy::prelude::*;
use bevy_renet::RenetServerPlugin;

use self::{
    components::ServerLobby,
    systems::{add_netcode_network, server_update_system},
};

pub mod components;
mod systems;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RenetServerPlugin)
            .insert_resource(ServerLobby::default());

        add_netcode_network(app);

        app.add_systems(Update, server_update_system);
    }
}
