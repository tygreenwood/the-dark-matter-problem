use bevy::prelude::*;
use bevy_renet::RenetClientPlugin;

use crate::setup::configs::AppStates;

use self::{
    components::{ClientLobby, Connected, NetworkMapping},
    systems::{add_netcode_network, client_send_input},
};

pub mod components;
mod systems;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RenetClientPlugin);

        add_netcode_network(app);

        app.insert_resource(ClientLobby::default())
            .insert_resource(NetworkMapping::default())
            .add_systems(
                Update,
                (client_send_input)
                    .in_set(Connected)
                    .run_if(in_state(AppStates::Game)),
            );
    }
}
