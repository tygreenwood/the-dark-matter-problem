use bevy::prelude::*;
use bevy_renet::{
    client_connected,
    renet::{
        transport::{ClientAuthentication, NetcodeClientTransport, NetcodeTransportError},
        RenetClient,
    },
};
use local_ip_address::local_ip;
use std::{
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

use crate::{
    animations::components::AnimationIndices,
    client::components::CurrentClientId,
    player::{
        components::{ControlledPlayer, Player},
        configs::PLAYER_RUNNING_ANIMATION_PATH,
    },
    server::components::{NetworkedEntities, ServerChannel, ServerMessages},
    setup::configs::{connection_config, AppStates, PROTOCOL_ID},
};

use super::{
    components::{
        ClientChannel, ClientLobby, Connected, NetworkMapping, PlayerInfo, PlayerMessage,
    },
    configs::SERVER_ADDRESS,
};

pub fn add_netcode_network(app: &mut App) {
    app.add_plugins(bevy_renet::transport::NetcodeClientPlugin);

    app.configure_sets(Update, Connected.run_if(client_connected()));

    let client = RenetClient::new(connection_config());

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
    query_player_transform: Query<(&GlobalTransform, &TextureAtlasSprite), With<ControlledPlayer>>,
) {
    let (player, tex_atlas) = query_player_transform.single();
    let player_input = PlayerMessage {
        translation: player.translation().into(),
        flip: tex_atlas.flip_x,
        index: tex_atlas.index,
    };
    let input_message = bincode::serialize(&player_input).unwrap();

    client.send_message(ClientChannel::Input, input_message);
}

pub fn update_netcode_network(
    client_id_res: Res<CurrentClientId>,
    mut transport_res: ResMut<NetcodeClientTransport>,
) {
    let client_id = client_id_res.0;

    let server_addr = SocketAddr::new(SERVER_ADDRESS.parse().unwrap(), 42069);

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

pub fn client_sync_players(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut client: ResMut<RenetClient>,
    client_id_res: Res<CurrentClientId>,
    mut lobby: ResMut<ClientLobby>,
    mut network_mapping: ResMut<NetworkMapping>,
) {
    let client_id = client_id_res.0;

    while let Some(message) = client.receive_message(ServerChannel::ServerMessages) {
        let server_message: ServerMessages = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::PlayerCreate {
                entity,
                id,
                translation,
            } => {
                println!("Player {} connected.", id);
                if id.raw() != client_id {
                    let transform = Transform {
                        translation: Vec3::new(translation[0], translation[1], 0.0),
                        scale: Vec3::new(5., 5., 1.),
                        ..default()
                    };
                    let player_texture_handle = asset_server.load(PLAYER_RUNNING_ANIMATION_PATH);
                    let player_texture_atlas = TextureAtlas::from_grid(
                        player_texture_handle,
                        Vec2::new(19.0, 32.0),
                        7,
                        1,
                        None,
                        None,
                    );
                    let player_texture_atlas_handle = texture_atlases.add(player_texture_atlas);
                    let player_animation_indices = AnimationIndices { first: 0, last: 6 };

                    let player_entity = commands
                        .spawn((
                            SpriteSheetBundle {
                                texture_atlas: player_texture_atlas_handle,
                                transform,
                                ..default()
                            },
                            Player { id },
                            player_animation_indices,
                        ))
                        .id();

                    let player_info = PlayerInfo {
                        server_entity: entity,
                        client_entity: player_entity,
                    };

                    lobby.players.insert(id, player_info);
                    network_mapping.0.insert(entity, player_entity);
                }
            }
            ServerMessages::PlayerRemove { id } => {
                if let Some(PlayerInfo {
                    server_entity,
                    client_entity,
                }) = lobby.players.remove(&id)
                {
                    commands.entity(client_entity).despawn();
                    network_mapping.0.remove(&server_entity);
                }
            }
        }
    }

    while let Some(message) = client.receive_message(ServerChannel::NetworkedEntities) {
        let networked_entities: NetworkedEntities = bincode::deserialize(&message).unwrap();

        for i in 0..networked_entities.entities.len() {
            if let Some(entity) = network_mapping.0.get(&networked_entities.entities[i]) {
                let translation: Vec3 = networked_entities.translations[i].into();

                let transform = Transform {
                    translation,
                    scale: Vec3::new(5., 5., 1.),
                    ..default()
                };

                commands
                    .entity(*entity)
                    .insert(transform)
                    .insert(TextureAtlasSprite {
                        index: networked_entities.frame[i],
                        flip_x: networked_entities.flip[i],
                        ..default()
                    });
            }
        }
    }
}
