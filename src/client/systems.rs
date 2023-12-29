use bevy::prelude::*;
use bevy_renet::{
    client_connected,
    renet::{
        transport::{ClientAuthentication, NetcodeClientTransport, NetcodeTransportError},
        ConnectionConfig, RenetClient,
    },
};
use local_ip_address::local_ip;
use std::{
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

use crate::{
    client::components::CurrentClientId,
    player::components::ControlledPlayer,
    setup::configs::{AppStates, PROTOCOL_ID},
};

use super::components::{ClientChannel, Connected, PlayerTransform};

pub fn add_netcode_network(app: &mut App) {
    app.add_plugins(bevy_renet::transport::NetcodeClientPlugin);

    app.configure_sets(Update, Connected.run_if(client_connected()));

    let client = RenetClient::new(ConnectionConfig::default());

    let server_addr = SocketAddr::new(local_ip().unwrap(), 0);

    let local_addr = SocketAddr::new(local_ip().unwrap(), 0);
    let socket = UdpSocket::bind(local_addr).unwrap();

    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

    app.insert_resource(client);
    app.insert_resource(transport);
    app.insert_resource(CurrentClientId(client_id));

    // If any error is found we just panic
    #[allow(clippy::never_loop)]
    fn panic_on_error_system(mut renet_error: EventReader<NetcodeTransportError>) {
        for e in renet_error.read() {
            panic!("{}", e);
        }
    }

    app.add_systems(
        Update,
        panic_on_error_system.run_if(in_state(AppStates::Game)),
    );
}

pub fn client_send_input(
    mut client: ResMut<RenetClient>,
    query_player_transform: Query<&GlobalTransform, With<ControlledPlayer>>,
) {
    let player = query_player_transform.single();
    let player_input = PlayerTransform {
        translation: player.translation().into(),
    };
    let input_message = bincode::serialize(&player_input).unwrap();

    client.send_message(ClientChannel::Input, input_message);
}

pub fn update_netcode_network(
    client_id_res: Res<CurrentClientId>,
    mut transport_res: ResMut<NetcodeClientTransport>,
) {
    let client_id = client_id_res.0;

    let server_addr = SocketAddr::new(local_ip().unwrap(), 0);

    let local_addr = SocketAddr::new(local_ip().unwrap(), 0);
    let socket = UdpSocket::bind(local_addr).unwrap();

    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    *transport_res = transport;
}
